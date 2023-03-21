import React, { useEffect, useRef, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

import style from "./style.module.css";

import Surreal from "surrealdb.js";

const db = new Surreal('http://127.0.0.1:8000/rpc');

const CreateProject = () => {
    const nameRef = useRef();
    const [filePath, setFilePath] = useState();

    const openFile = async () => {  
        // Open a selection dialog
        const path = await open({
            multiple: false,
            title: 'Open dataset',
            filters: [{
                name: '*',
                extensions: ['csv'],
            }]
        });
        if (path === null) return;
        setFilePath(path);
        // const result = await invoke('read_columns', { path });
    };

    const createProjectDB = async () => {
        const name = nameRef.current.value;
        // filePath
        const date = new Date()
        await db.signin({
            user: 'root',
            pass: 'root'
        });
        await db.use('main', 'config');
        const project = await db.create('project', { name, filePath, created: date, updated: date });
        db.close();
        const result = await invoke('create_project', { id: project.id, filePath: project.filePath });
        console.log(result);
    };

    useEffect(() => {
        invoke('resize', { w: 600, h: 400});
        return () => {
            db.close();
        };
      }, []);
    
    return (
        <div className="container">
            <main className={style.createProject}>
                <h2>Create project</h2>
                <input type="text" name="name" placeholder={`Project ${Math.floor((Math.random() * 999))}`} ref={nameRef}/>
                <p>{filePath}</p>
                <button onClick={openFile}>Select dataset</button>
                <button onClick={createProjectDB}>Create</button>
            </main>
        </div>
    );
};

export default CreateProject;
