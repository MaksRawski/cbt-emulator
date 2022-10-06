import { CPUModule } from "./Modules";

export class Flags extends CPUModule<{}>{
    name = "FLAGS"
    module() {
        return (
            <div>
                <p style={{ fontSize: "1.7em", margin: 0 }} className="row" id={this.name}></p>
                <p style={{ fontSize: "1.25em", margin: 0, fontWeight: "bold" }} className="row">C H O Z</p>
            </div>
        )
    }
}
