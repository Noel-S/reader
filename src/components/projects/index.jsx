import React, { useEffect, useState } from "react";

import style from "./style.module.css";

import Surreal from "surrealdb.js";
import { dialog } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";

const db = new Surreal('http://127.0.0.1:8000/rpc');

const Projects = () => {
    const navigate = useNavigate();
    const [projects, setProjects] = useState([]);

    const createProject = () => {  
        navigate("/create", { replace: true });
    };

    const readFromDB = async () => {
        try {
            await db.signin({
                user: 'root',
                pass: 'root'
            });
            await db.use('main', 'config');
            const result = await db.select("project");
            if (result.length == 0) return;
            setProjects(result);
        } catch (error) {
            dialog.message(error, {
                type: 'error'
            })
        }
    };

    useEffect(() => {
        readFromDB();
        return () => {
            db.close();
        };
    }, []);
    
    return (
        <div className="container">
            <main className={style.openProject}>
                {/* <h1>Load project</h1> */}
                <ul>
                    <button onClick={createProject} className={style.button}>
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <g id="Iconly/Light/Plus">
                                <g id="Plus">
                                    <path id="Line_185" d="M12.0001 8.32733V15.6537" stroke="#4182e9" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path id="Line_186" d="M15.6666 11.9905H8.33325" stroke="#4182e9" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path id="Path" fill-rule="evenodd" clip-rule="evenodd" d="M16.6857 2H7.31429C4.04762 2 2 4.31208 2 7.58516V16.4148C2 19.6879 4.0381 22 7.31429 22H16.6857C19.9619 22 22 19.6879 22 16.4148V7.58516C22 4.31208 19.9619 2 16.6857 2Z" stroke="#4182e9" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                </g>
                            </g>
                        </svg>
                        Create project
                    </button> 
                    { projects.map((p, i) => (<li key={`project-${i+1}`}>Project {i}</li>)) }
                </ul>
            </main>
        </div>
    );
};

export default Projects;
