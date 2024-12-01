const { invoke } = window.__TAURI__.core;

let editor;
let saveButton;
let openButton;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let greeting = await invoke("greet", { name: "John" });

  console.log(greeting);
}

async function saveFile() {
  let wasSuccessful = await invoke("save_file", { fileContents: editor.value });

  console.log("File save status: " + wasSuccessful);
}

async function openFile() {
  editor.value = await invoke("open_file", { fileName: "Test.txt" });
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
