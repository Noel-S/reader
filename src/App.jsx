import React from "react";
import { Routes, Route } from "react-router-dom";

import "./App.css";

import Projects from "./components/projects";
import Sidebar from "./components/sidebar";
import Content from "./components/content";
import CreateProject from "./components/create_project";
import Loading from "./components/loading";

function App() {
  return (
    <Routes>
      <Route path="/loading" element={<Loading />} />
      <Route path="/main" element={
        <div className="container">
          <Sidebar />
          <Content />
        </div>
      } />
      <Route path="/" element={<Projects />} />
      <Route path="/create" element={<CreateProject />} />
    </Routes>
  );
}

export default App;
