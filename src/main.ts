import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function connectTCP() {
  try {
    const response = await invoke<string>("tcp_client");
    console.log('Response from TCP server:', response);
  } catch (error) {
    console.error('Error invoking TCP client:', error);
  }
}

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    greet();

    await connectTCP();

    console.log(document.body.innerHTML);
    //document.location.href = "/pages/home.html";
  });
});
