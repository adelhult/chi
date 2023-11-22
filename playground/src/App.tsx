import { Editor, OnChange } from "@monaco-editor/react";
import {useState, useEffect, useMemo} from "react";
import init, {run} from "chi_web";
import styled from "styled-components";
import Convert from "ansi-to-html";

const MainView = styled.main`
  display: flex;
  flex-direction: column;
  width:100%;
`;

const Nav = styled.nav`
  box-sizing: border-box;
  overflow: auto;
  height: 70px;
  padding-left: 1rem;
  border-bottom: 1px solid #beb3a8;
  background: #FFFCF9;
  display: flex;
  align-items: center;
  gap: 3rem;

  & > ol {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    gap: 1.5rem;
    font-size: 0.9rem;
  }
 
`;

const Logo = styled.div`
  display: inline-flex;
  align-items: center;
  font-size: 1.5rem;
  gap: 0.3rem;
  
  & .chi {
    position: relative;
    font-family: 'Noto Serif', serif;
    font-size: 3rem;
    bottom: 0.6rem;
  }
`

const Output = styled.pre`
  overflow: auto;
  box-sizing: border-box;
  position: relative;
  border-top: 1px solid #beb3a8;
  padding: 0.5rem;
  padding-left: 1rem;
  height: 225px;
  width: 100%;
  margin:0;
  font-size: 0.9rem;
`;

const SAMPLE_PROGRAM = `
-- Welcome to the χ playground!

-- Small example program
-- Note: meta variables and assignments (let name = <some expr>;) are supported 
let add = rec add = \\x. \\y. case x of
{ Zero() -> y
; Suc(n) -> Suc(add n y)
};

let zero = Zero();

let three = Suc(Suc(Suc(Zero())));

let equals = rec equals = \\m. \\n. case m of
{ Zero() -> case n of
  { Zero() -> True()
  ; Suc(n) -> False()
  }
; Suc(m) -> case n of
  { Zero() -> False()
  ; Suc(n) -> equals m n
  }
};

equals (add zero three) three
-- the value of the last expression is printed (each time the contents of the editor changes) in window below
`;

function App() {
  const convert = useMemo(() => new Convert(), []);

  useEffect(() => {
    init().then(() => {
      setWasmLoaded(true)
    });
  }, []);

  
  const editorChange: OnChange = (value, event) => {
    try {
      const result = run(value ?? " ");
      setOutput(result);
    } catch (error) {
      setOutput(convert.toHtml((error as string) ?? ""));
    }
  };


  const [output, setOutput] = useState("");
  const [wasmLoaded, setWasmLoaded] = useState(false);
  return wasmLoaded && (
    <>
    <Nav>
      <Logo>
        <span className="chi">χ</span>
        playground
      </Logo>
      <ol>
        <a href="https://www.cse.chalmers.se/~nad/listings/chi/README.html"><li>Agda implementation</li></a>
        <a href="https://chalmers.instructure.com/courses/26348/file_contents/course%20files/reading/The_language_chi.pdf"><li>Description (PDF)</li></a>
        <a href="https://github.com/adelhult/chi"><li>Playground source code</li></a>
      </ol>
    </Nav>
    <MainView>
    <Editor height="calc(100vh - 70px - 225px)" width="100vw" defaultLanguage="" defaultValue={SAMPLE_PROGRAM} onChange={editorChange}/>
    <Output dangerouslySetInnerHTML={{__html: output}}>

    </Output>
    </MainView>

    </>
  )
}

export default App
