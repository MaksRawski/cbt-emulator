import { ReactNode } from "react";
import { CPUModule } from "./Modules";

// these interfaces represent the exact same structure
// as the one in src/lcd.rs
interface Cursor {
    visible: boolean,
    blinking: boolean
    row: number,
    column: number
}

interface Display {
    on: boolean,
    two_line_mode: boolean,
    buffer: Uint8Array,
}

export interface LCDState {
    cursor: Cursor,
    display: Display
}

export class LCD extends CPUModule<{}, LCDState>{
    name = "LCD"
    constructor(props: any) {
        super(props);
        this.state = {
            cursor: {
                visible: false,
                blinking: false,
                row: 0,
                column: 0,
            },
            display: {
                on: false,
                two_line_mode: false,
                buffer: new Uint8Array(80),
            }
        };
    }
    componentDidMount(): void {
        global.set_lcd = (v: LCDState) => {
            this.setState(v);
        }
    }
    content(row: number): ReactNode {
        let s = [];
        for (let i = 0; i <= 16; i++) {
            s.push(String.fromCharCode(this.state.display.buffer[i + 40 * row]));
        }
        if (this.state.cursor.visible && this.state.cursor.row === row) {
            s[this.state.cursor.column] = <span className="cursor blinking"></span>
        }
        return (
            <div>
                {s}
            </div>
        );
    }
    module() {
        return (
            <div className="LCD">
                <div className="LCD-row">{this.content(0)}</div>
                <div className="LCD-row">{this.content(1)}</div>
            </div>
        );
    }
}
