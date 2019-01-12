import JSONTree from 'react-json-tree';
import React from 'react';
import ReactDOM from 'react-dom';
import './style.css';
import * as three from 'three';
import { OrbitControls } from 'three-orbitcontrols-ts';

const render_div = document.getElementById('fighter-render');;
const width = render_div.offsetWidth;
const height = width;

const scene = new three.Scene();

const camera = new three.PerspectiveCamera(40, width / height, 1.0, 1000);
camera.position.set(0, 8, 30);

const controls = new OrbitControls(camera);
controls.target.set(0, 8, 0);
controls.update();

const renderer = new three.WebGLRenderer({ alpha: true });
renderer.setSize(width, height);
renderer.setClearColor(0xFFFFFF, 0);
render_div.appendChild(renderer.domElement);

for (let hurt_box of fighter_frame_data_json.hurt_boxes) {
    const geometry = new three.BoxGeometry(1, 1, 1);
    const material = new three.MeshBasicMaterial({ color: 0xffff00 });
    const cube = new three.Mesh(geometry, material);
    const diameter = hurt_box.hurt_box.radius * 2.0;

    const translation = new three.Matrix4();
    translation.makeTranslation(hurt_box.hurt_box.stretch.x + hurt_box.hurt_box.offset.x,
                                hurt_box.hurt_box.stretch.y + hurt_box.hurt_box.offset.y,
                                hurt_box.hurt_box.stretch.z + hurt_box.hurt_box.offset.z);

    const transform = new three.Matrix4();
    const bm = hurt_box.bone_matrix;
    //transform.set(bm.x.x, bm.x.y, bm.x.z, bm.x.w,
    //              bm.y.x, bm.y.y, bm.y.z, bm.y.w,
    //              bm.z.x, bm.z.y, bm.z.z, bm.z.w,
    //              bm.w.x, bm.w.y, bm.w.z, bm.w.w);
    console.log(transform);
    transform.set(bm.x.x, bm.y.x, bm.z.x, bm.w.x,
                  bm.x.y, bm.y.y, bm.z.y, bm.w.y,
                  bm.x.z, bm.y.z, bm.z.z, bm.w.z,
                  bm.x.w, bm.y.w, bm.z.w, bm.w.w);
    transform.multiply(translation);

    const orientation = new three.Quaternion();
    orientation.setFromRotationMatrix(transform); // TODO: transform3??!!??!

    cube.matrixAutoUpdate = false;
    cube.matrix.copyPosition(transform);
    //cube.matrix.makeRotationFromQuaternion(orientation);
    scene.add(cube);
}

function animate() {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
}
animate();

ReactDOM.render(
    <JSONTree data={fighter_scripts_json} />,
    document.getElementById('fighter-scripts')
);
