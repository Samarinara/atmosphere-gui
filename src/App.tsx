import React, { useState, useEffect, Suspense, lazy } from 'react';
import { Route, Link, BrowserRouter, Routes, Navigate } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import "./App.css";

import LoginPage from "./pages/login";
import HomePage from "./pages/home.tsx";

function App() {
  const [currentRoute, setCurrentRoute] = useState("home");
  return(
    <div>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Navigate to="/login" replace />}></Route>
        <Route path="/login" element={<LoginPage />}></Route>
        <Route path="/home" element={<HomePage />}></Route>
      </Routes>
      
      <div className="footer">
        <Link to="/home">Home</Link>
        <Link to="/login">Login</Link>
      </div>
    </BrowserRouter>
    </div>
  );
}

function RouteButton(text: string, route: string){
  return(
    <button
    onClick={() => {
    }}
    >{text}</button>
  )
}


export default App;
