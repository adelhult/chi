import { Editor, OnChange } from "@monaco-editor/react";
import {useState, useEffect} from "react";
import init, {run} from "chi_web";
import styled from "styled-components";

const MainView = styled.main`
  display: flex;
  width:100%;
`;

const Nav = styled.nav`
  width: 100%;
  height: 150px;
  box-sizing: border-box;
  overflow: auto;
  padding: 1rem;

  border-bottom: 1px solid black;

  & > ol {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    gap: 1rem;
  }
`;

const SAMPLE_PROGRAM = `
let foo = rec foo = \\m. \\n. case m of
{ Zero() -> case n of
  { Zero() -> True()
  ; Suc(n) -> False()
  }
; Suc(m) -> case n of
  { Zero() -> False()
  ; Suc(n) -> foo m n
  }
};

foo Suc(Zero()) Suc(Suc(Zero()))
`;

function App() {
  useEffect(() => {
    init().then(() => {
      setWasmLoaded(true)
    });
  }, []);
  
  const editorChange: OnChange = (value, event) => {
    const result = run(value ?? "");
    setOutput(result);
  };


  const [output, setOutput] = useState("");
  const [wasmLoaded, setWasmLoaded] = useState(false);
  return wasmLoaded && (
    <>
    <Nav>
      <h1>χ playground</h1>
      <ol>
        <a href="https://www.cse.chalmers.se/~nad/listings/chi/README.html"><li>Agda implementation</li></a>
        <a href="https://chalmers.instructure.com/courses/26348/file_contents/course%20files/reading/The_language_chi.pdf"><li>Description (PDF)</li></a>
        <a href="https://github.com/adelhult/chi"><li>Playground source code</li></a>
      </ol>
    </Nav>
    <MainView>
    <Editor height="calc(100vh - 150px)" width="60vw" defaultLanguage="" defaultValue={SAMPLE_PROGRAM} onChange={editorChange}/>
    <div>
      {output}
    </div>
    </MainView>

    </>
  )
}

export default App
