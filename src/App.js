import React, { useEffect, useState } from "react";
import * as d3 from "d3";
import worker from "./convex-hull.worker";

const { convexHull } = worker();

const Chart = ({ width, height, data }) => {
  const [ch, setCh] = useState([]);
  useEffect(() => {
    if (data != null) {
      convexHull(data).then((ch) => {
        setCh(ch);
      });
    }
  }, [data]);
  if (data == null) {
    return null;
  }
  const path = d3.path();
  if (ch.length > 0) {
    path.moveTo(ch[0].x, ch[0].y);
    for (let i = 1; i < ch.length; ++i) {
      path.lineTo(ch[i].x, ch[i].y);
    }
    path.closePath();
  }
  return (
    <svg width={width} height={height}>
      <g>
        <path
          d={path.toString()}
          fill="green"
          stroke="#000"
          strokeWidth="2"
          opacity="0.5"
        />
      </g>
      <g>
        {data.map(({ x, y }, i) => {
          return (
            <g key={i} transform={`translate(${x},${y})`}>
              <circle r="3" opacity={0.8} />
            </g>
          );
        })}
      </g>
    </svg>
  );
};

const App = () => {
  const [data, setData] = useState(null);
  const [n, setN] = useState(100);
  const width = 400;
  const height = 400;
  useEffect(() => {
    const data = Array.from({ length: n }, () => {
      const r = 180 * Math.random();
      const t = Math.PI * 2 * Math.random();
      return {
        x: Math.floor(r * Math.cos(t) + width / 2),
        y: Math.floor(r * Math.sin(t) + height / 2),
      };
    });
    setData(data);
  }, [n, width, height]);
  return (
    <div>
      <section className="section">
        <div className="container">
          <div className="content">
            <h1>Convex Hull with Rust and WebAssembly</h1>
            <p>
              This is an implementation of Graham scan convex hull finding
              algorithm with Rust and WebAssembly.
            </p>
            <p>
              Source code is available{" "}
              <a href="https://github.com/likr-sandbox/convex-hull">here</a>.
            </p>
          </div>
        </div>
      </section>
      <section className="section">
        <div className="container">
          <div>
            <form
              onSubmit={(event) => {
                event.preventDefault();
                setN(+event.target.elements.n.value);
              }}
            >
              <div className="field">
                <label className="label">Number of points</label>
                <div className="control">
                  <input
                    className="input"
                    type="number"
                    name="n"
                    defaultValue={n}
                  />
                </div>
              </div>
            </form>
          </div>
          <div>
            <Chart width={width} height={height} data={data} />
          </div>
        </div>
      </section>
      <section className="footer">
        <div className="container">
          <div className="content">
            <p className="has-text-centered">&copy; 2020 Yosuke Onoue</p>
          </div>
        </div>
      </section>
    </div>
  );
};

export default App;
