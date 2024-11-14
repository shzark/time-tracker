import React, {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import ProcessCard from "./ProcessCard.tsx";
import "./App.css";

interface ProcessInfo {
    id: string,
    name: string,
    running_time_formatted: string,
    memory_formatted: number
}


const App: React.FC = () => {

    const [processes,setProcesses] = useState<ProcessInfo[]>([]);
    const [maxMemoryProcess,setMaxMemoryProcess] = useState<ProcessInfo | null>(null);
    const [maxRunningProcess,setMaxRunningProcess] = useState<ProcessInfo | null>(null);


    useEffect(() => {
        async function fetchData() {
            const processList = await invoke<ProcessInfo[]>("list_process");
            const maxMemory = await invoke<ProcessInfo>("max_memory");
            const maxRunning = await invoke<ProcessInfo>("max_running_process");

            setProcesses(processList);

            setMaxMemoryProcess(maxMemory);

            setMaxRunningProcess(maxRunning);
        }

        fetchData();
        const  interval = setInterval(fetchData,1000);
        return () => clearInterval(interval);
    },[]);

    return (
        <main className="container">
            {maxMemoryProcess && <ProcessCard title="Max Memory process" process ={maxMemoryProcess} />}
            {maxRunningProcess && <ProcessCard title="Max Running Time" process ={maxRunningProcess} />}

            <div className="process-list">
                {processes.map((process) => (

                    <div key={process.id} className="process-item" >
                        <span> {process.name}  (ID: {process.id}) </span>
                    </div>
                    ))}
            </div>
        </main>
    );
}
export default App;