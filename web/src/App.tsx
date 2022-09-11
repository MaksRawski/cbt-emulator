import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import init, { add, setup_logging } from 'cbt_emulator';


function App() {
    return (
        <div className="App">
            <header className="App-header">
                CBT emulator
            </header>
        </div>
    );
}

// n: u8
function to_binary(n: number): string {
    let empty = '○';
    let full = '●';
    let res = "";
    for (let i = 0; i < 8; i++) {
        res += n & (1 << i) ? full : empty;
    }
    return res;
}

export default App;
