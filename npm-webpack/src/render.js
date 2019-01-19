import * as three from 'three';
import OrbitControls from 'orbit-controls-es6';

const ANIMATION_FLAGS = {
    NONE:                      0x0,
    NO_OUT_TRANSITION:         0x1,
    LOOP:                      0x2,
    MOVES_CHARACTER:           0x4,
    FIXED_TRANSLATION:         0x8,
    FIXED_ROTATION:            0x10,
    FIXED_SCALE:               0x20,
    TRANSITION_OUT_FROM_START: 0x40,
    UNKNOWN:                   0x80
};

export class FighterRender {
    constructor(action_data) {
        const render_div = document.getElementById('fighter-render');

        this.scene = new three.Scene();
        this.camera = new three.PerspectiveCamera(40, 1, 1.0, 1000);
        this.controls = new OrbitControls(this.camera, render_div);
        this.controls.target.set(0, 8, 0);
        this.controls.update();
        this.face_right();

        this.renderer = new three.WebGLRenderer({ alpha: true, antialias: true });
        this.renderer.setClearColor(0xFFFFFF, 0);
        render_div.appendChild(this.renderer.domElement);

        this.window_resize();
        window.addEventListener('resize', () => this.window_resize(), false);

        this.action_data = action_data;
        this.frame_index = 0;
        this.run = false;
        this.wireframe = false;
        this.material = new three.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.5 });

        this.setup_frame();
        this.animate();
    }

    wire_frame_toggle() {
        const button = document.getElementById('wire-frame-toggle');
        this.wireframe = !this.wireframe;
        if (this.wireframe) {
            button.innerHTML = "Transparent";
            this.material = new three.MeshBasicMaterial({ color: 0xffffff, transparent: true, wireframe: true });
        } else {
            button.innerHTML = "Wireframe";
            this.material = new three.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.5 });
        }
        this.setup_frame();
    }

    window_resize() {
        const render_div = document.getElementById('fighter-render');
        const width = render_div.offsetWidth;
        let height = width;
        if (height > 750) {
            height = 750;
        }

        this.camera.aspect = width / height;
        this.camera.updateProjectionMatrix();
        this.controls.update();
        this.renderer.setSize(width, height);
    }

    run_toggle() {
        if (this.run) {
            this.stop()
        }
        else {
            this.start()
        }
    }

    start() {
        const button = document.getElementById('run-toggle');
        button.innerHTML = "Stop";
        this.run = true;
    }

    stop() {
        const button = document.getElementById('run-toggle');
        button.innerHTML = "Run";
        this.run = false;
    }

    previous_frame() {
        this.stop();
        this.frame_index -= 1;
        if (this.frame_index == -1) {
            this.frame_index = this.action_data.frames.length - 1;
        }
        this.setup_frame();
    }

    next_frame() {
        this.stop();
        this.frame_index += 1;
        if (this.frame_index >= this.action_data.frames.length) {
            this.frame_index = 0;
        }
        this.setup_frame();
    }

    set_frame(index) {
        this.stop();
        this.frame_index = index;
        this.setup_frame();
    }

    face_left() {
        this.camera.position.set(40, 8, 0);
        this.controls.update();
    }

    face_right() {
        this.camera.position.set(-40, 8, 0);
        this.controls.update();
    }

    setup_frame() {
        const frame = this.action_data.frames[this.frame_index];

        // clear all objects from previous frame
        while (this.scene.children.length) {
            const child = this.scene.children[0];
            this.scene.remove(child);
            child.geometry.dispose();
        }

        // generate hurtboxes
        for (let hurt_box of frame.hurt_boxes) {
            const bm = hurt_box.bone_matrix;
            const bone_matrix = new three.Matrix4();
            bone_matrix.set(bm.x.x, bm.y.x, bm.z.x, bm.w.x,
                          bm.x.y, bm.y.y, bm.z.y, bm.w.y,
                          bm.x.z, bm.y.z, bm.z.z, bm.w.z,
                          bm.x.w, bm.y.w, bm.z.w, bm.w.w);
            const bone_scale = new three.Vector3();
            bone_scale.setFromMatrixScale(bone_matrix);

            const radius = hurt_box.hurt_box.radius;
            var stretch = hurt_box.hurt_box.stretch;
            var offset = hurt_box.hurt_box.offset;

            stretch = new three.Vector3(stretch.x, stretch.y, stretch.z);
            offset = new three.Vector3(offset.x, offset.y, offset.z);

            const stretch_face = stretch.clone();
            stretch_face.divideScalar(radius);
            stretch_face.divide(bone_scale);

            const vertices = [];
            const indices = [];

            var index_offset = 0;

            const widthSegments = 23; // needs to be odd, so we have a middle segment
            const heightSegments = 17; // needs to be odd, so we have a middle segment
            const grid = []
            // modified UV sphere generation from:
            // https://github.com/mrdoob/three.js/blob/4ca3860851d0cd33535afe801a1aa856da277f3a/src/geometries/SphereGeometry.js
            for (var iy = 0; iy <= heightSegments; iy++) {
                var verticesRow = [];
                var v = iy / heightSegments;

                for (var ix = 0; ix <= widthSegments; ix++) {
                    var u = ix / widthSegments;

                    // The x, y and z stretch values, split the sphere in half, across its dimension.
                    // This can result in 8 individual sphere corners.
                    var corner_offset = new three.Vector3();
                    if (u >= 0.25 && u <= 0.75) { // X
                        if (stretch.x > 0) {
                            corner_offset.x = stretch_face.x;
                        }
                    }
                    else if (stretch.x < 0) {
                        corner_offset.x = stretch_face.x;
                    }

                    if (v >= 0 && v <= 0.5) { // Y
                        if (stretch.y > 0) {
                            corner_offset.y = stretch_face.y;
                        }
                    }
                    else if (stretch.y < 0) {
                        corner_offset.y = stretch_face.y;
                    }

                    if (u >= 0 && u <= 0.5) { // Z
                        if (stretch.z > 0) {
                            corner_offset.z = stretch_face.z;
                        }
                    }
                    else if (stretch.z < 0) {
                        corner_offset.z = stretch_face.z;
                    }

                    // vertex generation is supposed have the 8 sphere corners take up exactly 1/8th of the unit sphere.
                    // However that is difficult because we would need to double up the middle segments.
                    // So instead we just make it look like this is the case by having large widthSegments and heightSegments.
                    const sin_v_pi = Math.sin(v * Math.PI);
                    vertices.push(corner_offset.x - Math.cos(u * Math.PI * 2) * sin_v_pi);
                    vertices.push(corner_offset.y + Math.cos(v * Math.PI));
                    vertices.push(corner_offset.z + Math.sin(u * Math.PI * 2) * sin_v_pi);

                    verticesRow.push(index_offset++);
                }
                grid.push(verticesRow);
            }

            for (var iy = 0; iy < heightSegments; iy++) {
                for (var ix = 0; ix < widthSegments; ix++) {
                    var a = grid[iy][(ix + 1) % widthSegments];
                    var b = grid[iy][ix];
                    var c = grid[iy + 1][ix];
                    var d = grid[iy + 1][(ix + 1) % widthSegments];

                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }

            const geometry = new three.BufferGeometry();
            geometry.addAttribute('position', new three.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            const cube = new three.Mesh(geometry, this.material);

            const transform_translation = new three.Matrix4();
            transform_translation.makeTranslation(hurt_box.hurt_box.offset.x / (bone_scale.x * radius),
                                                    hurt_box.hurt_box.offset.y / (bone_scale.y * radius),
                                                    hurt_box.hurt_box.offset.z / (bone_scale.z * radius));

            const transform_scale = new three.Matrix4();
            transform_scale.makeScale(radius, radius, radius);

            const transform = new three.Matrix4();
            transform.copy(bone_matrix);
            transform.multiply(transform_scale);
            transform.multiply(transform_translation);

            const orientation = new three.Quaternion();
            orientation.setFromRotationMatrix(transform);

            cube.matrixAutoUpdate = false;
            cube.matrix.copy(transform);
            this.scene.add(cube);
        }

        // update frame select
        for (var button of document.querySelectorAll('.frame-button')) {
            button.classList.remove('current-frame-button');
            if (parseInt(button.innerHTML, 10) == this.frame_index) {
                button.classList.add('current-frame-button');
            }
        }
    }

    animate() {
        if (this.run) {
            this.setup_frame();
            this.frame_index += 1;
            if (this.frame_index >= this.action_data.frames.length) {
                this.frame_index = 0;
            }
        }

        this.renderer.render(this.scene, this.camera);
        requestAnimationFrame(() => this.animate());
    }
}
