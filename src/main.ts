import { invoke } from "@tauri-apps/api/core";

const input = document.getElementById("pdfInput") as HTMLInputElement;
const iframe = document.getElementById("viewer") as HTMLIFrameElement;
const uploadBtn = document.getElementById("uploadBtn") as HTMLButtonElement;

async function loadInitialPdf() {
  try {
    const fileName = await invoke<string | null>("get_first_pdf_in_public");
    if (fileName) {
      iframe.src = `pdfs/${fileName}`;
      iframe.style.display = "block";
    } else {
      iframe.style.display = "none";
    }
  } catch {
    iframe.style.display = "none";
  }
}

input.addEventListener("change", async () => {
  const name = input.value.trim();
  if (!name) {
    iframe.style.display = "none";
    return;
  }

  try {
    const pdfPath = await invoke<string | null>("find_pdf_path_by_name", { name });

    if (pdfPath) {
      const fileName = pdfPath.split(/[\\/]/).pop(); // sadece dosya adı al
      iframe.src = `pdfs/${fileName}`;
      iframe.style.display = "block";
    } else {
      iframe.style.display = "none";
      alert("PDF bulunamadı.");
    }
  } catch {
    iframe.style.display = "none";
    alert("PDF aranırken bir hata oluştu.");
  }
});

uploadBtn.addEventListener("click", () => {
  alert("Buraya dosya yükleme fonksiyonu bağlanacak.");
});

loadInitialPdf();
