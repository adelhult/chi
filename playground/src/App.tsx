/* eslint-disable no-useless-escape */
import { Editor, OnChange, OnMount } from "@monaco-editor/react";
import {useState, useEffect, useMemo, useRef} from "react";
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
  height: 55px;
  padding-left: 1rem;
  background:#27282c;
  color: #ffffff;
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
  font-size: 1.2rem;
  gap: 0.3rem;
  
  & .chi {
    position: relative;
    font-family: 'Noto Serif', serif;
    font-size: 2.3rem;
    bottom: 0.45rem;
  }
`

const Output = styled.div`
  background: #f8f7f4;
  box-sizing: border-box;
  position: relative;
  padding: 0;
  margin: 0;
  height: calc(100vh - 55px);
  width: 40vw;
  display: flex;
  flex-direction: column;
  padding-left: 0.5rem;
  padding-right: 0.5rem;
  border-left: 1px solid #d3d2d0;

  & div {
    padding-bottom: 0.5rem;
    padding-top: 0.5rem;
  }

  & pre {
    border-top: 1px solid #d3d2d0;
    overflow: auto;
    margin: 0;
    padding: 0;
    padding-top: 0.5rem;
    height: 100%;
    box-sizing: border-box;
    font-size: 0.9rem;

    & .error {
      color: #A00;
    }
  }
`;

const Options = styled.div`
  display: flex;
  gap: 1rem;
  align-items: center;

  & div {
    padding: 0;
  }

  & label {
    user-select: none;
  }
`

enum Printer {
  Concrete = "concrete", 
  Abstract = "abstract",
  Debug = "debug"
}

type PrinterOptionsProps = {
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
  value: Printer;
}

const PrinterOptions= ({onChange, value}: PrinterOptionsProps) => {
  return <Options>
    <div>
      <input
          type="radio"
          name="Concrete"
          value={Printer.Concrete as string}
          id="concrete"
          checked={value === Printer.Concrete}
          onChange={onChange}
        />
        <label htmlFor="concrete">Concrete</label>
    </div>

    <div>
      <input
        type="radio"
        name="Abstract"
        value={Printer.Abstract as string}
        id="abstract"
        checked={value === Printer.Abstract}
        onChange={onChange}
      />
      <label htmlFor="abstract">Abstract</label>
    </div>
    <div>
      <input
        type="radio"
        name="Debug"
        value={Printer.Debug as string}
        id="debug"
        checked={value === Printer.Debug}
        onChange={onChange}
      />
      <label htmlFor="debug">Debug</label>
    </div>
  </Options>
}


function App() {
  const convert = useMemo(() => new Convert(), []);

  useEffect(() => {
    // Load the wasm module
    init().then(() => {
      setWasmLoaded(true)
    });
  }, []);

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const editorRef = useRef<any>(null);

  const editorMount: OnMount = (editor, monaco) => {
    editorRef.current = editor;

    // register the Chi language
    // TODO: This should really move someplace else, but I'm having trouble with the
    // typescript definitions...
    monaco.languages.register({ id: "chi" });
    monaco.languages.setMonarchTokensProvider("chi", {
      keywords: [
        "case",
        "of",
        "rec",
        "let",
      ],
    
      operators: [
        "->",
        "=",
        "\\",
      ],
    
      // we include these common regular expressions
      symbols: /[=><!~?:&|+\-*\/\^%]+/,
    
      // The main tokenizer for our languages
      tokenizer: {
        root: [
          // keywords and variables
          [/[a-z_$][\w$]*/, {
            cases: { "@keywords": "keyword", "@default": "identifier" },
          }],
          [/[A-Z][\w\$]*/, "type.identifier"], // Constructors
    
          // whitespace
          { include: "@whitespace" },
    
          // delimiters and operators
          [/[{}()]/, "@brackets"],
          [/[<>](?!@symbols)/, "@brackets"],
          [/@symbols/, { cases: { "@operators": "operator", "@default": "" } }],
    
          [/[;,.]/, "delimiter"],
        ],
    
        whitespace: [
          [/(^--.*$)/, "comment"],
          [/[ \t\r\n]+/, "white"],
        ],
      },
    });

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

  const [output, setOutput] = useState("");
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const [printer, setPrinter] = useState(Printer.Concrete);
  
  const printerChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setPrinter(event.target.value as Printer);
  }

  useEffect(() => {
    if (editorRef.current === null) {
      return;
    }

    const text = editorRef.current.getValue();
    
    try {
      const result = run(text ?? " ", printer as string);
      setOutput(result);
    } catch (error) {
      setOutput(convert.toHtml((error as string) ?? ""));
    }
  }, [printer, convert]);

  const editorChange: OnChange = (value, event) => {
    try {
      const result = run(value ?? " ", printer as string);
      setOutput(result);
    } catch (error) {
      setOutput(convert.toHtml((error as string) ?? ""));
    }
  };

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
    <Editor
      height="calc(100vh - 55px)"
      width="60vw"
      defaultLanguage="chi"
      onChange={editorChange}
      onMount={editorMount}
      options={{minimap: {enabled: false}}}
    />
    <Output>
      <div>
        <strong>Output</strong>
        <PrinterOptions value={printer} onChange={printerChange}/>
      </div>
      <pre dangerouslySetInnerHTML={{__html: output}}></pre>
    </Output>
    </MainView>

    </>
  )
}

export default App
