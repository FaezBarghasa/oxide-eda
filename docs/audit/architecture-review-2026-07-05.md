# Oxide — Mimari & Kod Kalitesi İnceleme Raporu

> **Tarih:** 2026-07-05
> **Kapsam:** Tüm workspace (16 crate, ~188K satır Rust, iced 0.14 GUI)
> **Sürüm:** v0.13.0 (`dev` @ 56df4df)
> **Yöntem:** 16 boyutlu çok-ajanlı statik inceleme + her orta/üstü bulgu için
> düşmanca (adversarial) doğrulama. 181 ham bulgu → 172 elde tutuldu
> (96'sı bağımsız doğrulandı, 8'i çürütüldü). En yüksek etkili doğrulanmamış
> bulgular ayrıca elle teyit edildi.

Bu rapor Oxide'in **mimarisini, kod bakımını, okunabilirliğini, kullanım
senaryolarını ve UX/UI tasarımını** inceler. Güvenlik bulguları da tabloya
dahildir ancak raporun ağırlığı — talep edildiği üzere — mimari, bakım ve
kullanılabilirlik üzerindedir.

---

## 1. Özet (Executive Summary)

Oxide olgun bir kod tabanı: temiz crate ayrımı hedefi, atomik-yazma politikası
(HI-6), tema token sistemi, chrome-catalog ile hızlı UI iterasyonu, kapsamlı
regresyon testleri ve düzenli CI/release akışları mevcut. Ancak inceleme, hem
**mimari borç** hem de **veri kaybına yol açabilecek somut hatalar** ortaya
koydu. Öne çıkan üç tema:

1. **`oxide-app` monolitleşti.** Uygulama crate'i 110K satır; tek bir dosya
   (`app/dispatch/library.rs`) **10.357 satır** ve içinde **5.071 satırlık tek
   bir fonksiyon** (`apply_footprint_primitive_edit`) barındırıyor. Mesaj enum'ı
   440+ varyantlı, alt sistemler arasında derleyici tarafından zorlanan sınır
   yok. Bu, derleme süresini, birleştirme (merge) çakışmalarını ve yeni katkıcı
   için giriş bariyerini doğrudan artırıyor.

2. **İki tema tekrar tekrar çıkıyor: "sessiz veri kaybı" ve "sessiz hata".**
   Uygulamadan çıkışta kaydedilmemiş değişiklikler **hiç uyarı vermeden**
   kayboluyor; belirli karakterler (`\`, `"`, satır sonu) içeren bir şematik
   kaydedildiğinde dosya **bir daha açılamaz** hâle geliyor; kütüphane sunucusu
   üretim modunda **bellek-içi** veritabanı kullanıyor (yeniden başlatmada her
   şey siliniyor). Kaydetme/açma/kütüphane hataları yalnızca arka plandaki bir
   log'a düşüyor, kullanıcıya gösterilmiyor.

3. **UI thread'i bloke eden senkron I/O.** git2 commit'leri, dosya
   ayrıştırma, PDF rasterizasyonu ve ERC çalıştırma `update()` içinde senkron
   çalışıyor. iced'in `Task::perform` + `spawn_blocking` altyapısı projede
   mevcut (OAuth ve history paneli bunu kullanıyor) ama kütüphane ve dışa
   aktarma yollarında kullanılmıyor.

Ek olarak: klavye kısayollarının bir kısmı (`Ctrl+C/X/V/D`, `Ctrl+Shift+Z`,
`Shift+Alt+A`) `Ctrl+rakam` yakalayıcısı ve büyük/küçük harf uyuşmazlığı
yüzünden **hiç çalışmıyor**; ERC net türetimi **T-bağlantılarını yok sayıyor**
(tel ortasındaki her bağlantı ayrı net sayılıyor); GPU render crate'i
(`oxide-gfx`) uygulamada kullanılmayan **ölü kod** ve iced'inkiyle uyumsuz
ikinci bir wgpu/naga/cosmic-text yığınını binary'e ekliyor.

### Önem dağılımı

| Önem | Adet | Açıklama |
|------|------|----------|
| 🔴 Kritik | 6 (4 ayrık) | Veri kaybı veya normal kullanımdan erişilebilen bozulma |
| 🟠 Yüksek | 43 | Muhtemel hata veya ciddi mimari tehlike |
| 🟡 Orta | 84 | Sağlamlık / bakım açığı |
| ⚪ Düşük | 39 | Cila / iyileştirme |

> Not: Bazı bulgular birden çok boyutta ayrı ayrı tespit edildiği için (ör.
> çıkışta veri kaybı 3 kez, monolit dosya 4 kez) tablodaki toplam ile ayrık
> sorun sayısı farklıdır.

---

## 2. Kritik Bulgular (detay)

### K1 — Uygulamadan çıkışta / pencere kapatmada kaydedilmemiş değişiklikler uyarısız kayboluyor
**Dosyalar:** `crates/oxide-app/src/app/dispatch/mod.rs:230, :430`;
`crates/oxide-app/src/app/handlers/menu/file_commands.rs:50`

Pencere `X`'i `Message::CloseMainWindow → iced::window::close` çağırıyor;
`SecondaryWindowClosed(main)` doğrudan `iced::exit()` dönüyor. `File ▸ Exit` de
aynı yola gidiyor. Hiçbir yol `document_state.dirty_paths`'i kontrol etmiyor —
oysa başlık çubuğu aynı değeri "• N unsaved" olarak gösteriyor ve proje kapatma
akışının zaten bir "Save All / Discard / Cancel" modalı var
(`view/dialogs.rs:2516`). Sonuç: en sıradan kullanıcı eylemi (`X`'e tıklamak,
`Alt+F4`, `File ▸ Exit`) oturumdaki tüm kaydedilmemiş şematik/sembol/footprint/
proje düzenlemelerini anında yok ediyor.

**Öneri:** `CloseMainWindow` içinde ve bir `window::close_requests()`
subscription'ında `dirty_paths` boş değilse mevcut `ProjectCloseConfirm` modalını
göster; seçim çözülmeden `iced::exit()` çağırma. (Bulgu #8, #51, #118)

### K2 — Şematik kaydı belirli karakterlerde açılamaz dosya üretiyor
**Dosyalar:** `crates/oxide-types/src/format.rs:1149` (`write_tsv_section`),
`:114` (`encode_cell`), `:132` (`decode_cell`)

`write_tsv_section`, ham TSV gövdesini bir TOML çok-satırlı temel dizesine
(`content = """..."""`) gömüyor. Ama `encode_cell` ne ters eğik çizgiyi ne de
satır sonunu kaçırıyor (escape), ve tırnakları ikizleyerek `"""` üretebiliyor.
Doğrulama (toml 0.8): `C:\Users\x` değeri → "invalid unicode 8-digit hex code",
`ab"` → parse hatası. Yani bir değer/referans/etiket alanına Windows yolu (`\`),
inç işareti (`1/4"`) veya yapıştırılmış satır sonu yazmak **başarıyla
kaydediliyor ama dosya bir daha açılamıyor** — kullanıcının şeması yok oluyor.
Ayrıca `encode_cell` düz `-` üretirken `decode_cell` bunu boş dizeye çeviriyor
(asimetri), tam `"-"` değerli bir MPN sessizce kayboluyor.

**Öneri:** `content` alanını elle `"""` bloğu yerine `toml::Value::String`
serileştirmesiyle üret; `encode_cell` ters eğik çizgi/satır sonu/tab'ı
kaçırsın veya reddetsin; `-` hücresini tırnakla. Bu karakterler için round-trip
testleri ekle. (Bulgu #50, #46, #59)

### K3 — Kütüphane sunucusu üretimde bellek-içi SQLite kullanıyor; yeniden başlatmada her şey siliniyor
**Dosyalar:** `crates/oxide-library-server/src/main.rs:53`,
`src/db/mod.rs:74`

`main.rs:53` her zaman `router_with_in_memory_state()` çağırıyor →
`sqlite::memory:` (`max_connections(1)`). Dosya/Postgres destekleyen
`AppState::connect` yalnızca testlerden çağrılıyor; binary hiçbir yerde
`DATABASE_URL` okumuyor. Bir ekip paylaşımlı kütüphane sunucusuna kaydettiği her
bileşen satırı, sembol, footprint ve simülasyon, süreç yeniden başladığında/
çöktüğünde/yeniden dağıtıldığında sessizce yok oluyor. Ayrıca `POST` satır ekleme
`ON CONFLICT DO UPDATE` ile mevcut satırın üzerine yazıp yine 201 dönüyor.

**Öneri:** `main.rs`'te `OXIDE_DATABASE_URL` oku, `AppState::connect + migrate`
kullan; test dışında bellek-içine düşüyorsa başlamayı reddet veya yüksek sesle
uyar. `POST`'u mevcut kayıtta 409 yap; `PUT/DELETE`'e iyimser kilit ekle.
(Bulgu #70, #71)

### K4 — glTF içe aktarıcı keyfi yerel dosya okuyor (path traversal)
**Dosyalar:** `crates/oxide-3d-model-importer/src/gltf/wrap.rs:138, :197`

`read_buffer_payload` ve `embed_image_uris`, glTF JSON'undan gelen (saldırgan
kontrolündeki) `uri`'yi `base_dir.join(uri)` ile birleştirip `std::fs::read`
yapıyor. `Path::join` mutlak bir yol (`/home/user/.ssh/id_rsa`) veya `../../..`
verildiğinde base dizininden çıkabiliyor. Bu dosyalar kütüphaneden indirilen
3B bileşen modelleri; kötü niyetli bir `.gltf` herhangi bir yerel dosyayı okuyup
üretilen/önbelleklenen GLB'ye base64 gömerek dışarı sızdırabilir.

**Öneri:** Şema içeren, `/` ile başlayan veya `..` bileşeni olan uri'leri
reddet; birleştirilmiş yolu canonicalize edip base_dir altında kaldığını
doğrula. Hem buffer'lar hem görüntüler için uygula. (Bulgu #165)

---

## 3. Mimari Değerlendirme (talep edilen ana odak)

**3.1. Monolit `oxide-app` ve mesaj yüzeyi.** Uygulama crate'i tüm alt
sistemleri (şematik editör, PCB, kütüphane tarayıcı, sembol/footprint editör,
dışa aktarma, tercihler) tek crate'te topluyor. En akut nokta
`app/dispatch/library.rs` (10.357 satır): içinde `apply_footprint_primitive_edit`
(5.071 satır), `apply_symbol_primitive_edit` (394 satır) ve `apply_inline_edit`
(451 satır) gibi saf editör-durum indirgeyicileri **dispatcher katmanında**
yaşıyor. Bunlar `crate::library::editor::{symbol,footprint}` altındaki durumlarına
taşınmalı; `sketch_dispatch.rs` bu deseni zaten gösteriyor. `PrimitiveEditorMsg`
tek enum'da Symbol* ve Footprint* varyantlarını karıştırdığından her iki
`apply_*` fonksiyonu diğerinin ~30-84 varyantını "no-op" olarak elle
listeliyor — enum ikiye bölünürse derleyici bütünlüğü (exhaustiveness) zorlar.

**3.2. Katman/bağımlılık yönü.** `oxide-types` yaprak kalıyor (iyi). Ancak
`oxide-gfx` iced'inkiyle uyumsuz **ayrı bir wgpu 29 / glyphon 0.11 / glam 0.32**
yığını pinliyor; dışarıdan yalnızca CPU tarafı (`primitive/scene/style`)
kullanılıyor, GPU pipeline'ları ölü. Binary'de **iki wgpu/naga ve iki
cosmic-text** derleniyor. `cargo tree -d` 115 çift-sürümlü paket raporluyor
ama CI `cargo deny check bans` çalıştırmadığından bu regresyon sessizce geçmiş.

**3.3. Nested dispatch ve Task kaybı.** `dispatch_update` düz varyant gruplaması
yapıp 8 alt-dispatcher'ı `unreachable!` ile koruyor; ~28 yerde iç içe
`self.update(...)` çağrısının döndürdüğü `Task` atılıyor (kuyruğa alınmış async
iş kayboluyor). Nested `Message::Ui(UiMessage)` gibi enum'lara geçmek hem
`unreachable!`'ları kaldırır hem de Task propagasyonunu netleştirir.

**3.4. Bağımlılık merkezileştirme.** `[workspace.dependencies]` var ama tutarsız
kullanılıyor: `oauth2` kök girdisi ölü (oxide-library kendi kopyasını
tanımlıyor), `dirs` iki crate'te ayrı, `oxide-output`/`chrome-catalog` path
bağımlılığı kullanıyor. Tek bir `iced_debug.log` repo köküne commit'lenmiş ve
`default-members` tanımlı değil (her `cargo build` sunucu+katalog dahil her şeyi
kuruyor).

---

## 4. Kullanım Senaryoları & UX/UI (talep edilen ikinci odak)

**4.1. Ölü klavye kısayolları.** `bootstrap.rs:758`'deki
`(Character(c), m) if m.command() && !m.alt()` yakalayıcı arm'ı, seçim-yuvası
tuşu değilse `Noop` dönüyor ve daha sonra gelen `Ctrl+C/X/V/D/Shift+V/Shift+G`
arm'larını gölgeliyor. Yani **kopyala/kes/yapıştır/çoğalt klavyeden hiç
çalışmıyor**. Ayrıca shift'li chord'lar (`Ctrl+Shift+Z`, `Shift+Alt+A`,
`Ctrl+Shift+S`) büyük harf `Character` beklerken iced küçük harf verdiği için
ölü. Yardım (F1) modalı bağlanmamış 5 tuş (F2, F11, T, R, Shift+F) reklamlıyor.

**4.2. Sessiz "no-op" UI öğeleri.** Annotate dialog'daki "All On / All Off /
Update Changes List" düğmeleri işlevsiz ve tıklanınca **dialog'u kapatıyor**;
komut paleti 5 komutu sessizce yutuyor; bazı menü öğeleri yalnızca log'a düşen
TODO'lar. Kullanıcı bir şey olmadığını anlamıyor.

**4.3. Onay ve geri bildirim eksikleri.** Editör sekmelerini kapatmak taslakları
onaysız siliyor (kodun kendi yorumu: "no dirty-park yet, so closing discards the
draft"); History panelinde "Restore this version" tek tıkla dosyanın üzerine
yazıyor; kaydet/aç ve kütüphane satır kaydetme hataları kullanıcıya hiç
gösterilmiyor. Uygulamanın zaten bir `export_error` modalı var — bu yollara da
bağlanmalı.

**4.4. Spec ile kod sapması.** `UX_REFERENCE_ALTIUM.md` kutu-seçim yönü
semantiği (L→R içeride, R→L kesişen) ve `Alt+tık` net vurgusu tanımlıyor ama
kod bunları uygulamıyor; `G` tuşu spec'teki döngü yerine grid seçici açıyor;
`F5` spec'te override toggle iken palet açıyor. Kısayol kayıt tablosu
(`shortcuts.rs`) gerçek işleyiciden ayrı tutuluyor ve senkronize değil.

**4.5. Tema & tutarlılık.** 216 sabit renk literal'i tema token'larını atlıyor;
beyaz-alfa dolgular açık temalarda kırılıyor. Özel tema token'ları her panel
yenilemesinde Oxide varsayılanlarına **sessizce sıfırlanıyor**. Sekme çubuğunda
taşma kaydırma, başlık kısaltma ve dirty göstergesi yok.

---

## 5. Performans (UI thread sıcak noktaları)

`update()`/`view()` içinde her karede tekrarlanan pahalı işler: Annotate dialog
açıkken **her karede** tüm kapalı sayfaları diskten okuyup ayrıştırıyor; boşta
fare hareketi `DragMove` + klonlar tetikliyor; `refresh_panel_ctx` her motor
mutasyonunda dosya sistemi stat/dizin okuması yapıyor; BOM önizleme her karede
tüm satırları String tahsisiyle yeniden sıralıyor; komut paleti açıkken katalogu
her karede yeniden kurup sıralıyor; PCB canvas her pan/zoom karesinde tüm kartı
yeniden tesselate ediyor. PDF rasterizasyonu ve ERC senkron çalışıp UI'ı
donduruyor. Çözüm deseni projede mevcut (`Task::perform` + `spawn_blocking`,
`HistoryLoaded` deseni) — bu yollara uygulanmalı.

---

## 6. Tam Bulgu Envanteri (boyuta göre)

Aşağıdaki tablolar 172 bulgunun tamamını içerir. "Durum" sütunu doğrulama
sonucudur; kritik/yüksek bulguların büyük çoğunluğu bağımsız doğrulanmıştır.


### Mimari & Bağımlılık Hijyeni

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | Two full wgpu/naga and cosmic-text stacks linked; oxide-gfx GPU code is dead | `crates/oxide-gfx/Cargo.toml:9; crates/oxide-gfx/src/lib.rs:7` |
| 🟠 Yüksek | oxide-app monolith concentrates in a 10,357-line library dispatch file | `crates/oxide-app/src/app/dispatch/library.rs:1; crates/oxide-app/src/panels/mod.rs:1` |
| 🟡 Orta | cargo-deny bans check never runs; 115 duplicate-versioned packages unpoliced | `.github/workflows/ci.yml:93; deny.toml:51` |
| 🟡 Orta | Library tracing warnings only reach logs via a Linux-only transitive feature | `crates/oxide-app/src/diagnostics.rs:55; crates/oxide-library/Cargo.toml:52` |
| ⚪ Düşük | Workspace dependency centralization is inconsistent; oauth2 workspace entry is dead | `Cargo.toml:82; crates/oxide-library/Cargo.toml:46` |
| ⚪ Düşük | oxide-renderer::iced_bridge is a dead phase-0 skeleton with no consumers | `crates/oxide-renderer/src/iced_bridge.rs:14; crates/oxide-renderer/src/lib.rs:7` |
| ⚪ Düşük | oxide-app exports all 26 top-level modules pub with no internal boundaries | `crates/oxide-app/src/lib.rs:8` |
| ⚪ Düşük | Stray tracked log file and missing workspace default-members | `iced_debug.log; Cargo.toml:3` |

### iced Çekirdeği (update/dispatch)

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🔴 Kritik | Main-window close and File-Exit discard unsaved edits with no prompt | `crates/oxide-app/src/app/dispatch/mod.rs:230; crates/oxide-app/src/app/dispatch/mod.rs:430` |
| 🟠 Yüksek | Global keyboard shortcuts mutate the main window's document while an undocked window is focused | `crates/oxide-app/src/app/bootstrap.rs:699; crates/oxide-app/src/app/contracts.rs:43` |
| 🟠 Yüksek | Synchronous git2 commits and repo init run on the UI thread inside update() | `crates/oxide-app/src/app/dispatch/library.rs:3703; crates/oxide-app/src/app/dispatch/library.rs:3613` |
| 🟠 Yüksek | Annotate dialog re-reads and re-parses every unopened sheet from disk on every frame | `crates/oxide-app/src/app/view/dialogs.rs:89; crates/oxide-app/src/app/view/dialogs.rs:3156` |
| 🟡 Orta | ~28 nested self.update() calls discard the returned Task, losing queued async work | `crates/oxide-app/src/app/dispatch/mod.rs:736; crates/oxide-app/src/app/dispatch/document.rs:30` |
| 🟡 Orta | Every idle mouse move dispatches DragMove and runs finish_update's clones | `crates/oxide-app/src/app/bootstrap.rs:861; crates/oxide-app/src/app/dispatch/ui.rs:32` |
| 🟡 Orta | refresh_panel_ctx does filesystem stats and directory reads on every engine mutation | `crates/oxide-app/src/app/runtime.rs:256; crates/oxide-app/src/app/runtime.rs:325` |
| 🟡 Orta | PDF export and print-preview rasterization run synchronously; every settings toggle re-rasterizes all pages | `crates/oxide-app/src/app/handlers/menu/export.rs:86; crates/oxide-app/src/app/handlers/menu/export.rs:569` |
| 🟡 Orta | RunErc clones every open document and parses all unopened sheets on the UI thread | `crates/oxide-app/src/app/handlers/erc.rs:98; crates/oxide-app/src/app/handlers/erc.rs:117` |
| 🟡 Orta | Manual variant grouping in dispatch_update backed by unreachable! panics in 8 sub-dispatchers | `crates/oxide-app/src/app/dispatch/mod.rs:19; crates/oxide-app/src/app/dispatch/ui.rs:326` |
| 🟡 Orta | dispatch/library.rs is a 10,357-line monolith over a 440-variant LibraryMessage | `crates/oxide-app/src/app/dispatch/library.rs:1; crates/oxide-app/src/library/messages.rs:34` |
| 🟡 Orta | Opening schematics, PCBs and libraries reads and parses files synchronously in update() | `crates/oxide-app/src/app/handlers/document_files.rs:214; crates/oxide-app/src/app/handlers/document_files.rs:224` |

### View Katmanı

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟡 Orta | needs_overlay predicate duplicates collect_overlays conditions and already omits export_error/print-preview | `crates/oxide-app/src/app/view/mod.rs:3871; crates/oxide-app/src/app/view/mod.rs:4620` |
| 🟡 Orta | Annotate dialog buttons 'Update Changes List', 'All On', 'All Off' silently close the dialog | `crates/oxide-app/src/app/view/dialogs.rs:532; crates/oxide-app/src/app/handlers/erc.rs:630` |
| 🟡 Orta | 216 hard-coded color literals bypass theme tokens; white-alpha fills break light themes | `crates/oxide-app/src/panels/properties_parameters.rs:643; crates/oxide-app/src/app/view/dialogs.rs:2359` |
| 🟡 Orta | Oversized view functions: collect_overlays 1178 lines, BOM body 840, element properties 780 | `crates/oxide-app/src/app/view/mod.rs:4611; crates/oxide-app/src/app/view/dialogs.rs:1673` |
| 🟡 Orta | BOM preview re-sorts all rows with per-comparison String allocations every frame | `crates/oxide-app/src/app/view/dialogs.rs:2088; crates/oxide-app/src/app/view/dialogs.rs:2106` |
| 🟡 Orta | Command palette rebuilds entire catalog and re-ranks it on every frame while open | `crates/oxide-app/src/app/view/mod.rs:4455; crates/oxide-app/src/app/command_palette.rs:78` |
| 🟡 Orta | Dead modal code still executes per frame: removed New Component and disabled Edit modal | `crates/oxide-app/src/app/view/mod.rs:5519; crates/oxide-app/src/app/view/mod.rs:5564` |
| 🟡 Orta | Modal backdrop and context-menu clamp blocks copy-pasted eight and five times respectively | `crates/oxide-app/src/app/view/mod.rs:5502; crates/oxide-app/src/app/view/mod.rs:5025` |
| 🟡 Orta | Element Properties panel round-trips typed values through display strings | `crates/oxide-app/src/panels/element_properties.rs:62; crates/oxide-app/src/panels/element_properties.rs:820` |
| ⚪ Düşük | Components and ERC panels render every list row per frame with per-row clones | `crates/oxide-app/src/panels/mod.rs:3378; crates/oxide-app/src/panels/mod.rs:4366` |
| ⚪ Düşük | Context submenu positions rely on hand-counted row constants that drift with menu edits | `crates/oxide-app/src/app/view/mod.rs:5075; crates/oxide-app/src/app/view/mod.rs:5199` |

### Canvas & Etkileşim

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | Ctrl+C/X/V/D shortcuts dead: shadowed by Ctrl+digit selection-slot catch-all | `crates/oxide-app/src/app/bootstrap.rs:758; crates/oxide-app/src/app/bootstrap.rs:27` |
| 🟠 Yüksek | Single global undo marker stack shared across per-document engines desyncs undo | `crates/oxide-app/src/app/state.rs:758; crates/oxide-app/src/app/mutation_gateway.rs:132` |
| 🟠 Yüksek | Find/Replace overwrites the entire text instead of the matched substring | `crates/oxide-app/src/app/handlers/find_replace.rs:49; crates/oxide-app/src/app/handlers/find_replace.rs:60` |
| 🟠 Yüksek | Switching between placement tools leaks wire/shape/arc buffers into the new tool | `crates/oxide-app/src/app/dispatch/tool.rs:480; crates/oxide-app/src/app/handlers/canvas.rs:760` |
| 🟠 Yüksek | Partial engine undo mutates document without repaint, marker desync on batch errors | `crates/oxide-app/src/app/mutation_gateway.rs:146; crates/oxide-app/src/app/mutation_gateway.rs:81` |
| 🟡 Orta | Ctrl+Z always undoes net-color floods first, regardless of edit order | `crates/oxide-app/src/app/handlers/editing_commands.rs:38; crates/oxide-app/src/app/state.rs:204` |
| 🟡 Orta | Double-click detection threshold is 3mm in world space, so it scales with zoom | `crates/oxide-app/src/canvas/mod.rs:356; crates/oxide-app/src/canvas/camera.rs:31` |
| 🟡 Orta | Selection not cleared when switching between two schematic tabs | `crates/oxide-app/src/app/load_gateway.rs:304; crates/oxide-app/src/app/load_gateway.rs:372` |
| 🟡 Orta | Engine undo history stores 100 full SchematicSheet clones per open document | `crates/oxide-engine/src/history.rs:11; crates/oxide-engine/src/history.rs:53` |
| ⚪ Düşük | DoubleClicked shape/polyline commits use unsnapped coordinates unlike single-click path | `crates/oxide-app/src/app/handlers/canvas.rs:1197; crates/oxide-app/src/app/handlers/canvas.rs:706` |
| ⚪ Düşük | Shift+Alt+A and Ctrl+Shift+Z shortcut arms unreachable due to case/order | `crates/oxide-app/src/app/bootstrap.rs:693; crates/oxide-app/src/app/bootstrap.rs:711` |
| ⚪ Düşük | Help modal advertises five shortcuts (T, R, F2, Shift+F, F11) with no implementation | `crates/oxide-app/src/shortcuts.rs:148; crates/oxide-app/src/app/bootstrap.rs:460` |

### Hata Yönetimi

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | atomic_write never fsyncs, so its documented power-loss guarantee does not hold | `crates/oxide-types/src/atomic_io.rs:40; crates/oxide-engine/src/lib.rs:86` |
| 🟠 Yüksek | TSV cell encode/decode asymmetry silently corrupts "-" values and bricks files containing newlines | `crates/oxide-types/src/format.rs:114; crates/oxide-types/src/format.rs:132` |
| 🟡 Orta | Project file saved with truncating fs::write, bypassing the workspace atomic-write policy | `crates/oxide-types/src/project.rs:365; crates/oxide-app/src/app/handlers/document_files.rs:887` |
| 🟡 Orta | Prefs write path panics UI thread when prefs.json root is a non-object JSON value | `crates/oxide-app/src/fonts.rs:1078` |
| 🟡 Orta | Failed primitive save still logs "[save] Wrote ..." success into diagnostics | `crates/oxide-app/src/app/handlers/document_files.rs:721; crates/oxide-app/src/app/dispatch/library.rs:3577` |
| ⚪ Düşük | Pad-array count expressions unbounded; a typo freezes or OOM-aborts the UI thread | `crates/oxide-bake/src/array/linear.rs:75; crates/oxide-bake/src/array/grid.rs:80` |
| ⚪ Düşük | Theme export swallows write errors and can silently write an empty file | `crates/oxide-app/src/app/handlers/preferences.rs:305; crates/oxide-app/src/app/handlers/preferences.rs:296` |

### Dosya Formatları & Kalıcılık

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🔴 Kritik | TSV cells embedded unescaped in TOML string; save produces unloadable schematic | `crates/oxide-types/src/format.rs:1149; crates/oxide-types/src/format.rs:114` |
| 🔴 Kritik | App exit never prompts for unsaved changes; all dirty documents silently lost | `crates/oxide-app/src/app/dispatch/mod.rs:430; crates/oxide-app/src/app/dispatch/mod.rs:230` |
| 🟠 Yüksek | '.snxprj' saved non-atomically and corrupted JSON silently degrades, then loss is persisted | `crates/oxide-types/src/project.rs:365; crates/oxide-types/src/project.rs:304` |
| 🟠 Yüksek | Project-close 'Save All' cannot save .snxprj or symbol/footprint drafts, blocking close | `crates/oxide-app/src/app/handlers/dock/project_navigation.rs:562; crates/oxide-app/src/app/handlers/dock/project_navigation.rs:584` |
| 🟡 Orta | atomic_write never fsyncs file or directory, weakening its documented crash-safety contract | `crates/oxide-types/src/atomic_io.rs:40` |
| 🟡 Orta | No autosave or crash recovery; a panic mid-update permanently drops the dirty engine | `crates/oxide-app/src/app/load_gateway.rs:106; crates/oxide-app/src/library/recovery.rs:1` |
| 🟡 Orta | No format migration path: exact version match plus silent unknown-field drop on rewrite | `crates/oxide-types/src/format.rs:1080; crates/oxide-types/src/format.rs:1673` |
| ⚪ Düşük | .snxpcb round-trip: pads keyed by ref designator merge under duplicates; layer types dropped | `crates/oxide-types/src/format.rs:1600; crates/oxide-types/src/format.rs:1717` |
| ⚪ Düşük | New-project/new-sheet writes bypass atomic_write; New Project truncates an existing .snxprj | `crates/oxide-app/src/app/handlers/document_files.rs:60; crates/oxide-app/src/app/handlers/dock/project_navigation.rs:958` |
| ⚪ Düşük | decode_cell silently converts a legitimate '-' cell to empty string | `crates/oxide-types/src/format.rs:131; crates/oxide-types/src/format.rs:114` |

### Kütüphane Alt Sistemi

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | All library mutations run blocking git2/file I/O and full rescans on UI thread | `crates/oxide-app/src/app/dispatch/library.rs:2043; crates/oxide-app/src/library/state.rs:729` |
| 🟠 Yüksek | No cross-process locking or staleness check: concurrent editors silently lose .snxlib updates | `crates/oxide-library/src/adapters/local_git.rs:90; crates/oxide-library/src/adapters/local_git.rs:583` |
| 🟠 Yüksek | mutate_library_file has no rollback: failed persist leaves memory and disk permanently divergent | `crates/oxide-library/src/adapters/local_git.rs:592; crates/oxide-library/src/library_file.rs:391` |
| 🟠 Yüksek | App primitive saves bypass adapter save_*; Stage 15 version cascade is dead code | `crates/oxide-app/src/app/dispatch/library.rs:3567; crates/oxide-library/src/adapters/local_git.rs:1214` |
| 🟠 Yüksek | dispatch/library.rs is 10,357 lines including one ~5,000-line function | `crates/oxide-app/src/app/dispatch/library.rs:4368; crates/oxide-app/src/app/dispatch/library.rs:9513` |
| 🟡 Orta | Library errors are swallowed into tracing::warn with no user-visible feedback | `crates/oxide-app/src/app/dispatch/library.rs:9994; crates/oxide-app/src/app/dispatch/library.rs:3577` |
| 🟡 Orta | Adapter sees only the first footprint/sim per container and only UUID-named files | `crates/oxide-library/src/adapters/local_git.rs:368; crates/oxide-library/src/adapters/local_git.rs:780` |
| 🟡 Orta | Every symbol lookup re-reads and re-parses the whole symbols directory | `crates/oxide-library/src/adapters/local_git.rs:637; crates/oxide-library/src/adapters/local_git.rs:1197` |
| 🟡 Orta | NewComponent always writes into open_libraries[0], ignoring which library is active | `crates/oxide-app/src/app/dispatch/library.rs:108; crates/oxide-app/src/library/commands.rs:460` |
| ⚪ Düşük | Symbol container saves use non-atomic fs::write, violating the adapter's own HI-6 policy | `crates/oxide-library/src/adapters/local_git.rs:687; crates/oxide-library/src/adapters/local_git.rs:498` |

### Kütüphane Sunucusu

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🔴 Kritik | Production server binary always uses in-memory SQLite; all data lost on restart | `crates/oxide-library-server/src/main.rs:53; crates/oxide-library-server/src/db/mod.rs:74` |
| 🟠 Yüksek | POST row insert silently upserts and no server-side concurrency control exists | `crates/oxide-library-server/src/db/mod.rs:161; crates/oxide-library-server/src/routes/rows.rs:45` |
| 🟠 Yüksek | No TLS anywhere: static bearer token crosses the network in cleartext | `crates/oxide-library-server/src/main.rs:45; crates/oxide-library/src/adapters/database.rs:56` |
| 🟡 Orta | Library bearer token stored plaintext in library.toml; keychain sigil never resolved | `crates/oxide-library/src/manifest.rs:39; crates/oxide-library/src/adapters/database.rs:71` |
| 🟡 Orta | Single shared token, no per-library authorization, and forgeable lock holder identity | `crates/oxide-library-server/src/lib.rs:126; crates/oxide-library-server/src/routes/locks.rs:68` |
| 🟡 Orta | Per-IP rate limiting collapses to one shared bucket behind a reverse proxy | `crates/oxide-library-server/src/lib.rs:63; crates/oxide-library-server/src/lib.rs:169` |
| ⚪ Düşük | Advisory locking is dead end-to-end; client holder is identical for all users | `crates/oxide-library/src/adapters/database.rs:77; crates/oxide-library-server/src/locks.rs:105` |
| ⚪ Düşük | Distributor URL host checks match attacker domains via bare suffix comparison | `crates/oxide-library/src/distributors/digikey.rs:404; crates/oxide-library/src/distributors/mouser.rs:182` |
| ⚪ Düşük | Server bearer token compared non-constant-time via deprecated ValidateRequestHeaderLayer::bearer | `crates/oxide-library-server/src/lib.rs:132` |
| ⚪ Düşük | Crate advertises 'HTTP+WS server' but no WebSocket endpoint exists | `crates/oxide-library-server/Cargo.toml:3; crates/oxide-library-server/src/lib.rs:1` |

### Çıktı Üretimi

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | PDF export panics (debug) or corrupts µ/°/± to '?' (release) | `crates/oxide-output/src/pdf/font.rs:142; crates/oxide-output/src/pdf/surface.rs:162` |
| 🟠 Yüksek | BOM CSV export vulnerable to spreadsheet formula injection | `crates/oxide-output/src/bom/csv.rs:45; crates/oxide-output/src/bom/csv.rs:24` |
| 🟡 Orta | HTML BOM writes custom column headers unescaped into <th> | `crates/oxide-output/src/bom/html.rs:99; crates/oxide-output/src/bom/mod.rs:97` |
| 🟡 Orta | "Current page" export/preview always renders the first project sheet, not the active one | `crates/oxide-app/src/app/handlers/menu/export.rs:992; crates/oxide-output/src/pdf/mod.rs:645` |
| 🟡 Orta | Print preview rasterizes every sheet eagerly, synchronously, and stores each RGBA buffer twice | `crates/oxide-app/src/app/handlers/menu/export.rs:569; crates/oxide-output/src/preview/rasterize.rs:24` |
| 🟡 Orta | BOM sorts references lexicographically: R10 before R2 | `crates/oxide-bom/src/lib.rs:194; crates/oxide-bom/src/lib.rs:252` |
| ⚪ Düşük | XLSX header format is created but never applied | `crates/oxide-output/src/bom/xlsx.rs:26; crates/oxide-output/src/bom/xlsx.rs:46` |
| ⚪ Düşük | Netlist menu drives a file dialog then always fails with NotImplemented | `crates/oxide-output/src/netlist/mod.rs:43; crates/oxide-app/src/app/handlers/menu/export.rs:109` |
| ⚪ Düşük | PDF text measured with Roboto metrics but rendered as Helvetica | `crates/oxide-output/src/pdf/font.rs:174; crates/oxide-output/src/pdf/font.rs:121` |

### GPU Render

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | PCB canvas rebuilds and retessellates entire board scene every pan/zoom frame | `crates/oxide-app/src/pcb_canvas.rs:440; crates/oxide-app/src/pcb_canvas.rs:453` |
| 🟡 Orta | GPU renderer is dead code: app draws on CPU, wgpu versions incompatible | `crates/oxide-gfx/Cargo.toml:9; crates/oxide-app/src/pcb_canvas.rs:299` |
| 🟡 Orta | Golden GPU regression tests never compare pixels and skip silently on CI | `crates/oxide-gfx/tests/regression_golden.rs:411; crates/oxide-gfx/tests/regression_golden.rs:377` |
| ⚪ Düşük | Polygon triangulation misrenders any contour whose vertex count is divisible by 3 | `crates/oxide-gfx/src/pipeline/polygon.rs:26; crates/oxide-renderer/src/pcb.rs:23` |
| ⚪ Düşük | GPU text pipeline cannot express camera pan and ignores rotation | `crates/oxide-gfx/src/pipeline/text.rs:13; crates/oxide-gfx/src/scene/upload.rs:19` |
| ⚪ Düşük | arc.wgsl renders full-circle and zero-sweep arcs as invisible | `crates/oxide-gfx/src/shader/arc.wgsl:37; crates/oxide-renderer/src/schematic.rs:172` |
| ⚪ Düşük | line.wgsl ignores style attribute; dashed ratsnest renders solid on GPU | `crates/oxide-gfx/src/shader/line.wgsl:20; crates/oxide-renderer/src/pcb.rs:536` |
| ⚪ Düşük | Scene.dirty is written but never read or cleared; three disconnected dirty mechanisms | `crates/oxide-renderer/src/schematic.rs:453; crates/oxide-renderer/src/pcb.rs:685` |
| ⚪ Düşük | cull_items rebuilds an R-tree and deep-clones polygons on every upload | `crates/oxide-gfx/src/scene/upload.rs:177; crates/oxide-gfx/src/scene/upload.rs:197` |
| ⚪ Düşük | Test-only unsafe env::set_var races concurrent getenv in multithreaded test runs | `crates/oxide-library-server/tests/integration_db.rs:36; crates/oxide-library-server/tests/primitives.rs:27` |

### Eşzamanlılık & Async

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | Concurrent git auto-commits to same repo race; per-instance mutex serializes nothing | `crates/oxide-app/src/app/handlers/document_files.rs:337; crates/oxide-library/src/adapters/local_git_project.rs:55` |
| 🟠 Yüksek | Save As and PDF export resolve the active tab at dialog-completion time | `crates/oxide-app/src/app/handlers/menu/file_commands.rs:23; crates/oxide-app/src/app/handlers/document_files.rs:918` |
| 🟠 Yüksek | Raw project_idx captured across dialog await; project close shifts Vec indexes | `crates/oxide-app/src/app/handlers/dock/project_navigation.rs:745; crates/oxide-app/src/app/handlers/dock/project_navigation.rs:971` |
| 🟡 Orta | PDF export and preview rasterization run synchronously on the UI update thread | `crates/oxide-app/src/app/handlers/menu/export.rs:86; crates/oxide-app/src/app/handlers/menu/export.rs:800` |
| 🟡 Orta | Cancelled DigiKey OAuth still persists refresh token; cancellation only drops the message | `crates/oxide-app/src/app/dispatch/library.rs:2740; crates/oxide-app/src/library/settings/digikey_oauth.rs:222` |
| 🟡 Orta | OAuth callback read breaks on Windows: accepted socket inherits nonblocking mode | `crates/oxide-app/src/library/settings/digikey_oauth.rs:175; crates/oxide-app/src/library/settings/digikey_oauth.rs:258` |
| 🟡 Orta | Primitive saves run git commits synchronously on the UI thread, racing the async pipeline | `crates/oxide-app/src/app/dispatch/library.rs:3703; crates/oxide-app/src/app/dispatch/library.rs:3613` |
| ⚪ Düşük | Single pending_pdf_options slot is clobbered by overlapping export dialogs | `crates/oxide-app/src/app/handlers/menu/export.rs:757; crates/oxide-app/src/app/handlers/menu/export.rs:56` |

### UX Senaryoları (spec uyumu)

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | Ctrl+1-8 catch-all arm swallows Ctrl+C/X/V/D keyboard shortcuts | `crates/oxide-app/src/app/bootstrap.rs:758; crates/oxide-app/src/app/bootstrap.rs:775` |
| 🟡 Orta | Shift-chorded shortcuts match lowercase chars: Ctrl+Shift+Z, Shift+Alt+A, Ctrl+Shift+S dead | `crates/oxide-app/src/app/bootstrap.rs:711; crates/oxide-app/src/app/bootstrap.rs:689` |
| 🟡 Orta | F1 shortcuts modal advertises five unbound keys: F2, F11, T, R, Shift+F | `crates/oxide-app/src/shortcuts.rs:130; crates/oxide-app/src/shortcuts.rs:241` |
| 🟡 Orta | Annotate dialog 'All On'/'All Off'/'Update Changes List' buttons close the dialog instead | `crates/oxide-app/src/app/view/dialogs.rs:531` |
| 🟡 Orta | Command palette lists five commands that silently no-op | `crates/oxide-app/src/app/command_palette.rs:329; crates/oxide-app/src/app/handlers/menu/view_commands.rs:35` |
| 🟡 Orta | Menu shortcut labels drift: Ctrl+Shift+P opens palette, Ctrl+N/O/Shift+S unbound | `crates/oxide-app/src/menu_bar.rs:371; crates/oxide-app/src/menu_bar.rs:412` |
| 🟡 Orta | Box-select direction semantics and Alt+click net highlight missing versus spec section 3 | `crates/oxide-app/src/canvas/mod.rs:500; crates/oxide-app/src/app/handlers/selection_workflow.rs:169` |
| 🟡 Orta | G key opens grid picker, contradicting spec cycle behavior and its own menu label | `crates/oxide-app/src/app/bootstrap.rs:525; crates/oxide-app/src/menu_bar.rs:516` |
| 🟡 Orta | Roadmap Phase 3/4 gate features absent at v0.13: measure, nudge, backspace wire-point | `docs/ROADMAP.md:342; docs/ROADMAP.md:304` |
| 🟡 Orta | Shortcut registry duplicated from key handler with no sync; modal omits many live bindings | `crates/oxide-app/src/shortcuts.rs:60; crates/oxide-app/src/app/bootstrap.rs:460` |
| ⚪ Düşük | F5 opens a net-color palette modal; spec defines F5 as an override toggle | `crates/oxide-app/src/app/bootstrap.rs:671; crates/oxide-app/src/app/dispatch/mod.rs:448` |

### UX Kalitesi

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🔴 Kritik | App quit and main-window close discard all unsaved work without any prompt | `crates/oxide-app/src/app/dispatch/mod.rs:230; crates/oxide-app/src/app/dispatch/mod.rs:430` |
| 🟠 Yüksek | Closing component/symbol/footprint editor tabs discards dirty drafts with no confirmation | `crates/oxide-app/src/app/handlers/document_tabs.rs:146; crates/oxide-app/src/app/dispatch/library.rs:2881` |
| 🟠 Yüksek | File save and open failures are never surfaced to the user, only logged | `crates/oxide-app/src/app/handlers/document_files.rs:86; crates/oxide-app/src/app/dispatch/library.rs:3256` |
| 🟠 Yüksek | History panel 'Restore this version' overwrites file on single click, no confirmation | `crates/oxide-app/src/panels/history.rs:179; crates/oxide-app/src/app/handlers/document_files.rs:414` |
| 🟠 Yüksek | Library component-row save failure is completely silent; Save button appears to work | `crates/oxide-app/src/app/dispatch/library.rs:3038` |
| 🟡 Orta | Print preview rasterizes all sheets synchronously in update(), freezing UI with no indicator | `crates/oxide-app/src/app/handlers/menu/export.rs:569; crates/oxide-app/src/app/handlers/menu/export.rs:800` |
| 🟡 Orta | Single-key canvas shortcuts stay live behind open modals; Esc misses several modals | `crates/oxide-app/src/app/bootstrap.rs:501; crates/oxide-app/src/app/bootstrap.rs:699` |
| 🟡 Orta | Custom theme tokens silently reset to Oxide defaults on every panel refresh | `crates/oxide-app/src/app/runtime.rs:549; crates/oxide-types/src/theme.rs:439` |
| 🟡 Orta | Esc/OS-close of Preferences bypasses unsaved guard and leaves theme preview half-applied | `crates/oxide-app/src/app/handlers/preferences.rs:23; crates/oxide-app/src/app/dispatch/mod.rs:249` |
| 🟡 Orta | PDF/BOM export silently drops sheets that fail to parse from disk | `crates/oxide-app/src/app/handlers/menu/export.rs:1008; crates/oxide-app/src/app/handlers/menu/export.rs:577` |
| 🟡 Orta | Enable Version Control can close reporting success while auto-commit stays disabled | `crates/oxide-app/src/app/handlers/dock/project_navigation.rs:341; crates/oxide-app/src/app/handlers/dock/project_navigation.rs:396` |
| ⚪ Düşük | Tab bar has no overflow scrolling, no title truncation, no dirty indicator | `crates/oxide-app/src/tab_bar.rs:35; crates/oxide-app/src/app/view/mod.rs:3814` |

### Test & CI

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | ERC rule engine has zero tests despite documented prior false-positive bug | `crates/oxide-erc/src/rules.rs:43; crates/oxide-erc/src/rules.rs:24` |
| 🟠 Yüksek | GPU golden tests silently pass when no adapter; CI has no Vulkan driver | `crates/oxide-gfx/tests/regression_golden.rs:377; crates/oxide-gfx/src/debug_pass.rs:900` |
| 🟠 Yüksek | Schematic editor undo/redo path is untested; only footprint-editor history is covered | `crates/oxide-engine/src/history.rs:25; crates/oxide-app/src/app/mutation_gateway.rs:134` |
| 🟡 Orta | Release workflow builds and publishes binaries without running any tests | `.github/workflows/release.yml:90` |
| 🟡 Orta | Clippy and rustfmt CI gates are decorative — they never fail the build | `.github/workflows/ci.yml:41; .github/workflows/ci.yml:77` |
| 🟡 Orta | Tests run only on Linux; Windows/macOS get compile checks despite being release targets | `.github/workflows/ci.yml:43; .github/workflows/release.yml:16` |
| 🟡 Orta | 5776-line flat regression.rs mixes six subsystems; grows monotonically per release | `crates/oxide-app/tests/regression.rs:1; crates/oxide-app/tests/regression.rs:52` |
| 🟡 Orta | Postgres backend never exercised in CI; only in-memory SQLite is tested | `crates/oxide-library-server/tests/integration_db.rs:444; .github/workflows/ci.yml:62` |
| 🟡 Orta | Server loopback-guard and rate limiter have no tests; test router bypasses governor | `crates/oxide-library-server/src/main.rs:16; crates/oxide-library-server/src/lib.rs:162` |
| 🟡 Orta | File-format negative tests skip field-level corruption; no fuzzing of hand-rolled parser | `crates/oxide-types/src/format.rs:301; crates/oxide-types/src/format.rs:2521` |
| ⚪ Düşük | oxide-widgets and chrome-catalog effectively untested; app handler layer thin on tests | `crates/oxide-widgets/src/tree_view.rs:1; crates/chrome-catalog/src/lib.rs:1` |

### Bakım & Okunabilirlik

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | apply_footprint_primitive_edit is a single 5,071-line function | `crates/oxide-app/src/app/dispatch/library.rs:4368` |
| 🟠 Yüksek | dispatch/library.rs god file: 10,357 lines mixing four responsibilities | `crates/oxide-app/src/app/dispatch/library.rs:29; crates/oxide-app/src/app/dispatch/library.rs:3794` |
| 🟠 Yüksek | PrimitiveEditorMsg couples symbol and footprint editors via giant no-op arms | `crates/oxide-app/src/library/messages.rs:1315; crates/oxide-app/src/app/dispatch/library.rs:4176` |
| 🟡 Orta | Twenty more god files; seven additional functions exceed 1,000 lines | `crates/oxide-app/src/app/view/mod.rs:4611; crates/oxide-app/src/library/editor/footprint/canvas/mod.rs:235` |
| 🟡 Orta | CI lint gates are advisory: clippy -W, fmt continue-on-error, no lint config | `.github/workflows/ci.yml:41; .github/workflows/ci.yml:77` |
| 🟡 Orta | allow(dead_code) on entire 100-plus-variant message enums hides unused variants | `crates/oxide-app/src/library/messages.rs:1314; crates/oxide-app/src/library/messages.rs:557` |
| 🟡 Orta | 89 copies of one hardcoded rgba; 203 inline button::Style literals bypass theming | `crates/oxide-app/src/library/browser.rs:877; crates/oxide-app/src/library/create_options.rs:122` |
| 🟡 Orta | User-visible menu items are silent TODO no-ops that only log | `crates/oxide-app/src/app/dispatch/library.rs:1127; crates/oxide-app/src/app/dispatch/library.rs:1366` |
| 🟡 Orta | polite_wait throttle logic copy-pasted across all four distributor adapters | `crates/oxide-library/src/distributors/digikey.rs:327; crates/oxide-library/src/distributors/mouser.rs:90` |
| 🟡 Orta | Two different structs both named FootprintEditorState force aliasing everywhere | `crates/oxide-app/src/app/documents.rs:306; crates/oxide-app/src/library/editor/footprint/state/mod.rs:61` |
| ⚪ Düşük | Translate widget duplicates 200 lines of Widget-delegation boilerplate from oxide-widgets | `crates/oxide-app/src/app/view/translate.rs:41; crates/oxide-widgets/src/tab_pill.rs:88` |
| ⚪ Düşük | Legacy standard_sym/standard_sch naming still live alongside snx* formats | `crates/oxide-widgets/src/tree_view.rs:200; crates/oxide-app/src/app/helpers.rs:62` |

### Alan Motorları (sketch/ERC/DSL)

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🟠 Yüksek | ERC union-find ignores T-junctions, splitting every mid-wire connection into separate nets | `crates/oxide-erc/src/context.rs:427; crates/oxide-erc/src/rules.rs:194` |
| 🟠 Yüksek | Deleting a sketch point leaves dangling line/arc refs, permanently breaking the solver | `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs:317; crates/oxide-sketch/src/solver/state.rs:95` |
| 🟠 Yüksek | DSL 'apply_to' and 'scope' clauses are parsed but silently never enforced | `crates/oxide-erc-dsl/src/compiler.rs:103; crates/oxide-erc/src/rule.rs:93` |
| 🟡 Orta | entity_colours passes empty params despite its own HI-14 comment, missing parametric over-constraints | `crates/oxide-sketch/src/solver/dof.rs:91; crates/oxide-sketch/src/solver/mod.rs:84` |
| 🟡 Orta | PowerPortShort only detects ports at identical coordinates, not wired shorts | `crates/oxide-erc/src/rules.rs:523` |
| 🟡 Orta | UnusedPin flags DoNotConnect/Unclassified pins, ignoring the precomputed 'required' flag | `crates/oxide-erc/src/rules.rs:43; crates/oxide-erc/src/context.rs:336` |
| 🟡 Orta | DuplicateRefDesignator is per-sheet only; cross-sheet duplicates in a project are never flagged | `crates/oxide-erc/src/rules.rs:84; crates/oxide-app/src/app/handlers/erc.rs:133` |
| 🟡 Orta | BadHierSheetPin silently skips validation when the child schematic is missing or basename-collided | `crates/oxide-erc/src/rules.rs:404; crates/oxide-app/src/app/handlers/erc.rs:73` |
| 🟡 Orta | NetLabelConflict only compares local Net labels, contradicting its documented global-vs-local scope | `crates/oxide-erc/src/rules.rs:203; crates/oxide-erc/src/lib.rs:46` |
| 🟡 Orta | DSL compile errors only reach a debug log; ERC then runs without user rules | `crates/oxide-app/src/app/handlers/erc.rs:201; crates/oxide-erc-dsl/src/parser.rs:52` |
| ⚪ Düşük | DSL net-rule diagnostics anchor at origin (0,0) with no selectable object | `crates/oxide-erc-dsl/src/compiler.rs:203; crates/oxide-app/src/app/handlers/erc.rs:354` |
| ⚪ Düşük | AnnotateAll logic duplicated in engine, and ResetOnly records history even when nothing changed | `crates/oxide-engine/src/lib.rs:730; crates/oxide-engine/src/annotation.rs:54` |

### İçe Aktarıcılar

| Önem | Bulgu | Dosya |
|------|-------|-------|
| 🔴 Kritik | glTF importer reads arbitrary local files via unsanitized buffer/image URIs | `crates/oxide-3d-model-importer/src/gltf/wrap.rs:138; crates/oxide-3d-model-importer/src/gltf/wrap.rs:197` |
| 🟠 Yüksek | VRML parser has unbounded recursion — nested nodes overflow the stack | `crates/oxide-3d-model-importer/src/vrml/parser.rs:250; crates/oxide-3d-model-importer/src/vrml/parser.rs:526` |
| 🟠 Yüksek | STEP entity line-counting is O(n²), hanging import on large files | `crates/oxide-3d-model-importer/src/step/p21.rs:152` |
| 🟡 Orta | STEP splitter breaks on ';' inside quoted string literals | `crates/oxide-3d-model-importer/src/step/p21.rs:150` |
| 🟡 Orta | Internet-facing importers have no malformed/truncated/fuzz tests | `crates/oxide-3d-model-importer/tests/import_integration.rs:1; crates/oxide-3d-model-importer/src/vrml/parser.rs:601` |
| ⚪ Düşük | GLB writer silently truncates payload length to u32 | `crates/oxide-3d-model-importer/src/glb/writer.rs:26` |
| ⚪ Düşük | compute_min_max panics on positions not divisible by 3 | `crates/oxide-3d-model-importer/src/normalize/mod.rs:167` |

---

## 7. Açılan GitHub Issue'ları (öneri indeksi)

Bu rapordaki tüm öneriler, ilgili alt sisteme göre gruplanmış 21 detaylı
issue'ya dönüştürülmüştür (2026-07-05, `dev`):

| Issue | Başlık | Önem | Alan |
|-------|--------|------|------|
| [#95](https://github.com/alplabai/oxide/issues/95) | Çıkışta kaydedilmemiş değişiklikler uyarısız kayboluyor | 🔴 | Veri kaybı |
| [#96](https://github.com/alplabai/oxide/issues/96) | Şematik kaydı açılamaz dosya üretiyor (TSV↔TOML) | 🔴 | Veri kaybı |
| [#97](https://github.com/alplabai/oxide/issues/97) | Sunucu üretimde bellek-içi SQLite kullanıyor | 🔴 | Veri kaybı |
| [#98](https://github.com/alplabai/oxide/issues/98) | oxide-app monolitini ayrıştır (10K/5K satır) | 🟠 | **Mimari** |
| [#99](https://github.com/alplabai/oxide/issues/99) | Mesaj yönlendirme: nested Task kaybı + unreachable! | 🟠 | **Mimari** |
| [#100](https://github.com/alplabai/oxide/issues/100) | oxide-gfx ölü GPU yığını + çift wgpu | 🟠 | **Mimari** |
| [#101](https://github.com/alplabai/oxide/issues/101) | Bağımlılık & workspace hijyeni | 🟡 | **Mimari** |
| [#102](https://github.com/alplabai/oxide/issues/102) | Canvas düzenleme doğruluğu (undo/tool/find-replace) | 🟠 | Canvas |
| [#103](https://github.com/alplabai/oxide/issues/103) | Klavye kısayolları ölü + spec/menü sapması | 🟠 | UX |
| [#104](https://github.com/alplabai/oxide/issues/104) | Kalıcılık sağlamlığı (fsync/atomik/autosave) | 🟠 | Kalıcılık |
| [#105](https://github.com/alplabai/oxide/issues/105) | Kütüphane: UI bloklama, kilit yok, rollback yok | 🟠 | Kütüphane |
| [#106](https://github.com/alplabai/oxide/issues/106) | Eşzamanlılık: dialog sonrası bayat çözümleme | 🟠 | Async |
| [#107](https://github.com/alplabai/oxide/issues/107) | ERC doğruluğu (T-bağlantı, DSL scope) | 🟠 | Motor |
| [#108](https://github.com/alplabai/oxide/issues/108) | Sketch: nokta silme dangling ref bırakıyor | 🟠 | Motor |
| [#109](https://github.com/alplabai/oxide/issues/109) | Çıktı: PDF non-ASCII, BOM injection, yanlış sayfa | 🟠 | Çıktı |
| [#110](https://github.com/alplabai/oxide/issues/110) | UX: sessiz hatalar + onaysız yıkıcı eylemler | 🟠 | UX |
| [#111](https://github.com/alplabai/oxide/issues/111) | Performans: kare başına disk I/O / re-parse | 🟠 | Perf |
| [#112](https://github.com/alplabai/oxide/issues/112) | 3B içe aktarıcı sağlamlığı (traversal/recursion/O(n²)) | 🟠 | İçe aktarma |
| [#113](https://github.com/alplabai/oxide/issues/113) | Test & CI boşlukları | 🟡 | Test |
| [#114](https://github.com/alplabai/oxide/issues/114) | Tema bypass + view katmanı kopya-yapıştır | 🟡 | Bakım |
| [#115](https://github.com/alplabai/oxide/issues/115) | Sunucu sertleştirme (TLS/token/kilit) | 🟡 | Sunucu |

**Mevcut ilgili issue'lar:** #52 (font cap-height sınırı — kalıcılık),
#63 (grafik/panel çakışması — layout), #94 (i18n — sabit dizeler), #91
(keyboard preset — kısayol), #92 (parça eşleme), #93 (global pin adresleme).

---

## 8. Öncelik Önerisi (yol haritası)

1. **Hemen (veri kaybı):** #95, #96, #97 — kullanıcı verisini koruyan üç kritik.
2. **Kısa vade (görünür hatalar):** #102, #103, #110 — bozuk kısayollar ve
   sessiz hatalar günlük kullanımı doğrudan etkiliyor; düşük risk, yüksek getiri.
3. **Mimari borç (planlı):** #98, #99, #100, #101 — monolit ayrıştırma ve
   bağımlılık düzeni; yeni özelliklerin hızını belirler. `PrimitiveEditorMsg`
   bölme ve `apply_*` taşıma en yüksek kaldıraç.
4. **Doğruluk & sağlamlık:** #104, #105, #106, #107, #108, #109, #112.
5. **Kalite altyapısı:** #113 (CI lint/test kapıları gerçek yapılmalı),
   #114 (tema), #111 (performans), #115 (sunucu).

> CI lint kapılarının (#113) erken sertleştirilmesi önemli: `clippy -D warnings`
> ve gerçek `cargo deny check bans` açık olsaydı bu rapordaki bulguların bir
> kısmı (ölü kod, çift bağımlılık, kullanılmayan varyantlar) zaten yakalanırdı.

---

_Rapor, 16 boyutlu çok-ajanlı statik inceleme + düşmanca doğrulama ile
üretildi. Tüm dosya:satır referansları v0.13.0 (`dev` @ 56df4df) anlıkına
göredir._
