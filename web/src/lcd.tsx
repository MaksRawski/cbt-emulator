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
    content(min_range: number, max_range: number): string {
        let s = "";
        for (let i = min_range; i < max_range; i++) {
            s += String.fromCharCode(this.state.display.buffer[i]);
        }
        return s;
    }
    module() {
        return <div className="LCD">
            <div className="LCD-row">{this.content(0, 16)}</div>
            <div className="LCD-row">{this.content(40, 56)}</div>
        </div>
    }
}
