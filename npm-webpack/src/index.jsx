import JSONTree from 'react-json-tree'
import React from 'react'
import ReactDOM from 'react-dom'
import './style.css'

ReactDOM.render(
    <JSONTree data={fighter_scripts_json} />,
    document.getElementById('fighter-scripts')
);
