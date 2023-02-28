import { Tooltip } from "@mui/material";
import { CPUModule } from "./Modules";

const OFF = '○';
const ON = '●';

// taken directly from emulator's `src/cw.rs`
const CW_LABELS = [
    "HLT", "LAI", "HAI", "MO", "II", "MI", "SR", "LPO", "LPI", "HPO", "PCC", "HPI", "AO", "AI",
    "CO", "CI", "AL3", "AL2", "AL1", "AL0", "ALO", "ALE", "ALM", "ALC", "BO", "BI", "DO", "DI",
    "LCM", "LCE", "SPO", "SPI",
];

const CW_TOOLTIPS = [
    "HALT", "LOW MEMORY ADDRESS IN", "HIGH MEMORY ADDRESS IN", "MEMORY OUT", "INSTRUCTION IN", "MEMORY IN", "STEP RESET",
    "LOW PROGRAM COUNTER OUT", "LOW PROGRAM COUNTER IN", "HIGH PROGRAM COUNTER OUT", "PROGRAM COUNTER COUNT", "HIGH PROGRAM COUNTER IN",
    "A REGISTER OUT", "A REGISTER IN", "C REGISTER OUT", "C REGISTER IN", "ALU S3", "ALU S2", "ALU S1", "ALU S0",
    "ALU OUT", "ALU ENABLE", "ALU MODE", "ALU CARRY IN", "B REGISTER OUT", "B REGISTER IN", "D REGISTER OUT", "D REGISTER IN",
    "LCD MODE", "LCD ENABLE", "STACK POINTER OUT", "STACK POINTER IN"
];

export class CW extends CPUModule<{}, { cw: number }>{
    name = "CW"
    description = "CONTROL WORD"

    constructor(props: any) {
        super(props);
        this.state = { cw: 0 };
    }
    componentDidMount(): void {
        global.set_cw = (v: number) => {
            this.setState({ cw: v });
        }
    }
    cw_to_labels(cw: number) {
        let cws: [boolean?] = [];

        for (let i = 0; i < 32; i++) {
            cws[i] = ((cw & (1 << i)) > 0)
        }
        return (
            <div id="CW">
                {cws.map((v, i) => {
                    return (
                        <Tooltip followCursor title={CW_TOOLTIPS[i]}>
                            <div className="cb" key={i}>
                                <div className="LED">{v ? ON : OFF}</div>
                                <div style={{ visibility: v ? "visible" : "hidden" }} className="cb-label">
                                    {CW_LABELS[i]}
                                </div>
                            </div>
                        </Tooltip>
                    )
                })
                }
            </div >);
    }
    module() {
        return this.cw_to_labels(this.state.cw);
    }
}
