import { CPUModule } from "./Modules";

const OFF = '○';
const ON = '●';

// taken directly from emulator's `src/cw.rs`
const CW_LABELS = [
    "HLT", "LAI", "HAI", "MO", "II", "MI", "SR", "LPO", "LPI", "HPO", "PCC", "HPI", "AO", "AI",
    "CO", "CI", "AL3", "AL2", "AL1", "AL0", "ALO", "ALE", "ALM", "ALC", "BO", "BI", "DO", "DI",
    "LCM", "LCE", "SPO", "SPI",
];

export class CW extends CPUModule<{}, { cw: number }>{
    name = "CW"
    description = "control word"

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
                        <div className="cb" key={i}>
                            <div className="LED">{v ? ON : OFF}</div>
                            <div style={{ visibility: v ? "visible" : "hidden" }} className="cb-label">{CW_LABELS[i]}</div>
                        </div>
                    )
                })}
            </div>);
    }
    module() {
        return this.cw_to_labels(this.state.cw);
    }
}
