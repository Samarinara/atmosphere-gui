import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [handle, setHandle] = useState("");
  const [password, setPassword] = useState("");

  async function login() {

    invoke("login", { uname: handle, pwd: password });
    
  }



  return (
    <main className="container">
      <h1>Welcome to the Atmosphere</h1>

      <div className="row">
        <a href="https://bsky.app" target="_blank">
          <img src="/src/assets/bluesky.svg" className="logo bsky" alt="Bluesky logo" />
        </a>
      </div>
      <p>Click on the butterfly if you have never used bluesky before to create an account</p>
      <p></p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          login();
        }}
      >
        <input
          id="uname-input"
          onChange={(e) => setHandle(e.currentTarget.value)}
          placeholder="Enter a your handle (@bob.bsky.social)"
        />
        <button type="submit">Login</button>
        
      </form>

        <p></p>

      <form className="row">
      <input
          id="pwd-input"
          onChange={(e) => setPassword(e.currentTarget.value)}
          placeholder="Enter a your password"
        />
      </form>
      
      <p>{handle}</p>
      <p>{password}</p>
    </main>
  );
}

export default App;
