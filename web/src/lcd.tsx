import { CPUModule } from "./Modules";

// these interfaces represent the exact same structure
// as the one in src/lcd.rs
interface Cursor {
    visible: boolean,
    blinking: boolean
    position: number,
}

interface Display {
    on: boolean,
    two_line_mode: boolean,
    chars: Uint8Array,
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
                position: 0,
            },
            display: {
                on: false,
                two_line_mode: false,
                chars: new Uint8Array(80),
            }
        };
    }
    componentDidMount(): void {
        global.set_lcd = (v: LCDState) => {
            this.setState(v);
        }
    }
    module() {
        return <div className="LCD">
            <div className="LCD-row"></div>
            <div className="LCD-row"></div>
        </div>
    }
}
