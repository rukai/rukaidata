import JSONTree from 'react-json-tree';
import React from 'react';
import ReactDOM from 'react-dom';
import './style.css';
import { FighterRender } from './render.js';

window.fighter_render = new FighterRender(fighter_action_data);

ReactDOM.render(
    <JSONTree data={fighter_action_data.scripts} />,
    document.getElementById('fighter-scripts')
);
