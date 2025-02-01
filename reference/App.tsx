import "./styles.css";

type VotolRawFrame = number[];

type VotolSerialFrameProps = {
  frame: VotolRawFrame,
}

export function VotolSerialFrame(props: VotolSerialFrameProps) {
  return (
    <ol>
      {props.frame.map(n => <li>{n.toString(16).toUpperCase()}</li>)}
    </ol>
  );
}

const EXAMPLE_FRAME_STR = "c9 14 02 53 48 4f 57 00 00 00 a00 00 aa 00 00 00 18 aa 00 00 00 00 c4 0d";
const EXAMPLE_FRAME = EXAMPLE_FRAME_STR.split(' ').map(b => parseInt(b, 16));

export default function App() {
  return (
    <div className="App">
      <VotolSerialFrame frame={EXAMPLE_FRAME} />
    </div>
  );
}
