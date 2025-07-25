import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const input = document.getElementById("pdfInput") as HTMLInputElement;
const iframe = document.getElementById("viewer") as HTMLIFrameElement;
const uploadBtn = document.getElementById("uploadBtn") as HTMLButtonElement;

(async () => {
  await invoke("ensure_pdf_folder_exists");
})();

let basePdfPath: string;

async function getBasePath() {
  basePdfPath = await invoke<string>("get_base_pdf_dir");
}

async function showPdf(filePath: string) {
  try {
    const base64 = await invoke<string>("read_pdf_as_base64", {
      path: filePath,
    });
    const binary = atob(base64);
    const len = binary.length;
    const bytes = new Uint8Array(len);
    input.value = "";

    for (let i = 0; i < len; i++) {
      bytes[i] = binary.charCodeAt(i);
    }

    const blob = new Blob([bytes], { type: "application/pdf" });
    const url = URL.createObjectURL(blob);
    iframe.src = url;
    iframe.style.display = "block";
  } catch (err) {
    console.error("PDF gösterim hatası:", err);
    iframe.style.display = "none";
  }
}

async function loadInitialPdf() {
  try {
    const fileName = await invoke<string | null>("get_first_pdf_in_public");
    if (fileName) {
      const fullPath = `${basePdfPath}/${fileName}`;
      await showPdf(fullPath);
    } else {
      iframe.style.display = "none";
    }
  } catch (err) {
    console.error("Başlangıçta PDF gösterilemedi:", err);
    iframe.style.display = "none";
  }
}

input.addEventListener("change", async () => {
  let name = input.value.trim();
  if (!name) {
    iframe.style.display = "none";
    return;
  }

  // Eğer uzantı yoksa otomatik olarak ".pdf" ekle
  if (!name.toLowerCase().endsWith(".pdf")) {
    name += ".pdf";
  }

  try {
    const pdfPath = await invoke<string | null>("find_pdf_path_by_name", {
      name,
    });

    if (pdfPath) {
      await showPdf(pdfPath);
    } else {
      iframe.style.display = "none";
      alert("PDF bulunamadı.");
    }
  } catch (err) {
    iframe.style.display = "none";
    alert("PDF aranırken bir hata oluştu.");
    console.error(err);
  }
});

uploadBtn.addEventListener("click", async () => {
  const selected = await open({
    multiple: false,
    filters: [{ name: "PDF Files", extensions: ["pdf"] }],
  });

  if (typeof selected === "string") {
    try {
      await invoke("copy_pdf_to_public", { path: selected });
      alert("PDF başarıyla yüklendi.");
      await loadInitialPdf();
    } catch (err) {
      alert("Yükleme sırasında hata oluştu: " + err);
      console.error(err);
    }
  }
});

// input hep fokus kalsın
input.focus();
setInterval(() => input.focus(), 2000);

await getBasePath();
await loadInitialPdf();
