const { invoke } = window.__TAURI__.core;

let editor;
let saveButton;
let openButton;

async function saveFile() {
  let wasSuccessful = await invoke("save_file_via_dialog", { fileContents: editor.value });
}

async function openFile() {
  editor.value = await invoke("open_file_via_dialog");
}

window.addEventListener("DOMContentLoaded", () => {
  editor = document.querySelector("#editor");
  saveButton = document.querySelector("#save_file_button");
  openButton = document.querySelector("#open_file_button");





  saveButton.addEventListener("click", (e) => {
    saveFile();
  });

  openButton.addEventListener("click", (e) => {
    openFile();
  });
  
});
