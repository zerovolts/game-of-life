import React, { Component } from 'react';
import './App.css';

class App extends Component {
  constructor(props) {
    super(props)
    this.state = {
      grid: [[]]
    }

    this.fetchGrid = this.fetchGrid.bind(this)
    this.fetchRandomize = this.fetchRandomize.bind(this)
    this.fetchStep = this.fetchStep.bind(this)
    this.fetchClear = this.fetchClear.bind(this)
    this.fetchGlider = this.fetchGlider.bind(this)
    this.fetchReload = this.fetchReload.bind(this)
    this.fetchSet = this.fetchSet.bind(this)
  }

  fetchGlider() {
    this.fetchSet(7, 6)
    this.fetchSet(7, 7)
    this.fetchSet(7, 8)
    this.fetchSet(6, 8)
    this.fetchSet(5, 7)
  }

  fetchReload() {
    this.fetchGrid()
  }

  fetchGrid() {
    fetch("http://localhost:4000/")
      .then(res => res.json())
      .then(data => {
        this.setState({
          grid: data
        })
      })
  }

  fetchRandomize() {
    fetch("http://localhost:4000/randomize", {method: "POST"})
      .then(() => {
        this.fetchGrid()
      })
  }

  fetchStep() {
    fetch("http://localhost:4000/step", {method: "POST"})
      .then(() => {
        this.fetchGrid()
      })
  }

  fetchClear() {
    fetch("http://localhost:4000/clear", {method: "POST"})
      .then(() => {
        this.fetchGrid()
      })
  }

  fetchSet(x, y) {
    fetch(`http://localhost:4000/${x}/${y}`, {method: "POST"})
      .then(() => {
        this.fetchGrid()
      })
  }

  componentDidMount() {
    this.fetchGrid()
  }

  render() {
    const grid = this.state.grid.map((row, y) =>
      <div>
        {row.map((element, x) =>
          <div className={"cell " + (element ? "alive" : "dead")}
            onClick={() => this.fetchSet(x, y)}></div>
        )}
      </div>
    )
    //console.log(this.state.grid)

    return (
      <div>
        <div className="cell-grid">
          {grid}
        </div>
        <button onClick={this.fetchRandomize}>Randomize</button>
        <button onClick={this.fetchStep}>Step</button>
        <button onClick={this.fetchClear}>Clear</button>
        <button onClick={this.fetchGlider}>Glider</button>
        <button onClick={this.fetchReload}>Reload</button>
      </div>
    )
  }
}

export default App;
