import { Editor, OnChange, OnMount } from "@monaco-editor/react";
import {useState, useEffect, useMemo} from "react";
import init, {run} from "chi_web";
import styled from "styled-components";
import Convert from "ansi-to-html";
import readGist from "./gist";
import WELCOME_TEXT from "./welcomeText";

const MainView = styled.main`
  display: flex;
  flex-direction: row;
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

const Output = styled.div`
  box-sizing: border-box;
  position: relative;
  padding: 0;
  margin: 0;
  height: calc(100vh - 70px);
  width: 40vw;
  display: flex;
  flex-direction: column;
  padding-left: 0.5rem;
  padding-right: 0.5rem;

  & div {
    padding-bottom: 0.5rem;
    padding-top: 0.5rem;
  }

  & pre {
    border-top: 1px solid #beb3a8;
    overflow: auto;
    margin: 0;
    padding: 0;
    padding-top: 0.5rem;
    height: 100%;
    box-sizing: border-box;
    font-size: 0.9rem;
  }
`;

function App() {
  const convert = useMemo(() => new Convert(), []);

  useEffect(() => {
    // Load the wasm module
    init().then(() => {
      setWasmLoaded(true)
    });
  }, []);

  const editorMount: OnMount = (editor, monaco) => {
    // Once the editor has mounted, check if there is a gist id in the url
    // if so, load the gist otherwise show the welcome text
    const params = new URLSearchParams(window.location.search);
    const gistId = params.get("gist");
    if (gistId) {
      const text = readGist(gistId).then(text => editor.setValue(text));
    } else {
      editor.setValue(WELCOME_TEXT);
    }
  }
  
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
    <Editor height="calc(100vh - 70px)" width="60vw" defaultLanguage="" onChange={editorChange} onMount={editorMount}/>
    <Output>
      <div>Output</div>
      <pre dangerouslySetInnerHTML={{__html: output}}></pre>
    </Output>
    </MainView>

    </>
  )
}

export default App
