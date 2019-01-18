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
        this.material = new three.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.5 });

        this.setup_frame();
        this.animate();
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

    // TODO
    // The current geometry generation is taken exactly from brawlbox.
    // However the brawlbox code is really inefficient.
    // We could improve it by generating each vertex only once for the edge generation.
    // Then the face and side generation should no longer create any vertices, just link the vertices created by edge generation.
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

            let left_offset = 1;
            if (stretch.x > 0) {
                left_offset = (stretch.x / radius + 1) / bone_scale.x;
            }

            let right_offset = -1;
            if (stretch.x < 0) {
                right_offset = (stretch.x / radius - 1) / bone_scale.x;
            }

            let top_offset = 1;
            if (stretch.y > 0) {
                top_offset = (stretch.y / radius + 1) / bone_scale.y;
            }

            let bottom_offset = -1;
            if (stretch.y < 0) {
                bottom_offset = (stretch.y / radius - 1) / bone_scale.y;
            }

            let front_offset = 1;
            if (stretch.z > 0) {
                front_offset = (stretch.z / radius + 1) / bone_scale.z;
            }

            let back_offset = -1;
            if (stretch.z < 0) {
                back_offset = (stretch.z / radius - 1) / bone_scale.z;
            }

            const vertices = [
                // left face
                left_offset, 0.0,            0.0,
                left_offset, stretch_face.y, 0.0,
                left_offset, stretch_face.y, stretch_face.z,
                left_offset, 0.0,            stretch_face.z,

                // right face
                right_offset, 0.0,            0.0,
                right_offset, stretch_face.y, 0.0,
                right_offset, stretch_face.y, stretch_face.z,
                right_offset, 0.0,            stretch_face.z,

                // top face
                0.0,            top_offset, 0.0,
                0.0,            top_offset, stretch_face.z,
                stretch_face.x, top_offset, stretch_face.z,
                stretch_face.x, top_offset, 0.0,

                // bottom face
                0.0,            bottom_offset, 0.0,
                0.0,            bottom_offset, stretch_face.z,
                stretch_face.x, bottom_offset, stretch_face.z,
                stretch_face.x, bottom_offset, 0.0,

                // front face
                0.0,            0.0,            front_offset,
                stretch_face.x, 0.0,            front_offset,
                stretch_face.x, stretch_face.y, front_offset,
                0.0,            stretch_face.y, front_offset,

                // back face
                0.0,            0.0,            back_offset,
                stretch_face.x, 0.0,            back_offset,
                stretch_face.x, stretch_face.y, back_offset,
                0.0,            stretch_face.y, back_offset,
            ];

            const indices = [
                0, 1, 2,
                0, 2, 3,

                6, 5, 4,
                6, 4, 7,

                8, 9, 10,
                8, 10, 11,

                14, 13, 12,
                14, 12, 15,

                18, 17, 16,
                18, 16, 19,

                20, 21, 22,
                20, 22, 23,
            ];

            var index_offset = 24;

            // generate eight corners: XYZ, XYz, XyZ, Xyz, xYZ, xYz, xyZ, xyz
            const resolution = 16;
            const angle_iterations = 360 / resolution
            for (var quadrant = 0; quadrant < 8; quadrant++) {
                for (var i = 0; i < 180 / angle_iterations; i++) {
                    const ring_angle1 = (i * angle_iterations) / 180 * Math.PI;
                    const ring_angle2 = ((i + 1) * angle_iterations) / 180 * Math.PI;

                    for (var j = 0; j < 360 / angle_iterations; j++) {
                        const angle1 = (j * angle_iterations) / 180 * Math.PI;
                        const angle2 = ((j + 1) * angle_iterations) / 180 * Math.PI;

                        var q = 0;
                        var corner_offset = new three.Vector3();

                        if (Math.cos(angle2) >= 0) { // X
                            q += 4;
                            if (stretch.x > 0) {
                                corner_offset.x = stretch_face.x;
                            }
                        }
                        else if (stretch.x < 0) {
                            corner_offset.x = stretch_face.x;
                        }

                        if (Math.sin(angle2) >= 0) { // Y
                            q += 2;
                            if (stretch.y > 0) {
                                corner_offset.y = stretch_face.y;
                            }
                        }
                        else if (stretch.y < 0) {
                            corner_offset.y = stretch_face.y;
                        }

                        if (Math.cos(ring_angle2) >= 0) { // Z
                            q += 1;
                            if (stretch.z > 0) {
                                corner_offset.z = stretch_face.z;
                            }
                        }
                        else if (stretch.z < 0) {
                            corner_offset.z = stretch_face.z;
                        }

                        if (quadrant == q) {
                            vertices.push(corner_offset.x + Math.cos(angle1) * Math.sin(ring_angle2));
                            vertices.push(corner_offset.y + Math.sin(angle1) * Math.sin(ring_angle2));
                            vertices.push(corner_offset.z + Math.cos(ring_angle2));

                            vertices.push(corner_offset.x + Math.cos(angle2) * Math.sin(ring_angle2));
                            vertices.push(corner_offset.y + Math.sin(angle2) * Math.sin(ring_angle2));
                            vertices.push(corner_offset.z + Math.cos(ring_angle2));

                            vertices.push(corner_offset.x + Math.cos(angle2) * Math.sin(ring_angle1));
                            vertices.push(corner_offset.y + Math.sin(angle2) * Math.sin(ring_angle1));
                            vertices.push(corner_offset.z + Math.cos(ring_angle1));

                            vertices.push(corner_offset.x + Math.cos(angle1) * Math.sin(ring_angle1));
                            vertices.push(corner_offset.y + Math.sin(angle1) * Math.sin(ring_angle1));
                            vertices.push(corner_offset.z + Math.cos(ring_angle1));

                            indices.push(index_offset);
                            indices.push(index_offset+1);
                            indices.push(index_offset+2);

                            indices.push(index_offset);
                            indices.push(index_offset+2);
                            indices.push(index_offset+3);
                            index_offset += 4;
                        }
                    }
                }
            }

            // generate edges
            for (var i = 0; i < 360 / angle_iterations; i++) {
                // x-axis edges
                const ang1 = (i * angle_iterations) / 180 * Math.PI;
                const ang2 = ((i + 1) * angle_iterations) / 180 * Math.PI;

                var z1 = Math.cos(ang1);
                var z2 = Math.cos(ang2);
                var y1 = Math.sin(ang1);
                var y2 = Math.sin(ang2);

                var x1 = stretch.x < 0 ? stretch_face.x : 0;
                var x2 = stretch.x > 0 ? stretch_face.x : 0;

                if (y2 >= 0 && stretch.y > 0)
                {
                    y1 += stretch_face.y;
                    y2 += stretch_face.y;
                }
                if (y2 <= 0 && stretch.y < 0)
                {
                    y1 += stretch_face.y;
                    y2 += stretch_face.y;
                }
                if (z2 >= 0 && stretch.z > 0)
                {
                    z1 += stretch_face.z;
                    z2 += stretch_face.z;
                }
                if (z2 <= 0 && stretch.z < 0)
                {
                    z1 += stretch_face.z;
                    z2 += stretch_face.z;
                }

                vertices.push(x1);
                vertices.push(y1);
                vertices.push(z1);

                vertices.push(x2);
                vertices.push(y1);
                vertices.push(z1);

                vertices.push(x2);
                vertices.push(y2);
                vertices.push(z2);

                vertices.push(x1);
                vertices.push(y2);
                vertices.push(z2);

                indices.push(index_offset);
                indices.push(index_offset+1);
                indices.push(index_offset+2);

                indices.push(index_offset);
                indices.push(index_offset+2);
                indices.push(index_offset+3);
                index_offset += 4;

                // y-axis edges
                var x1 = Math.cos(ang1);
                var x2 = Math.cos(ang2);
                var z1 = Math.sin(ang1);
                var z2 = Math.sin(ang2);

                var y1 = stretch.y < 0 ? stretch_face.y : 0;
                var y2 = stretch.y > 0 ? stretch_face.y : 0;

                if (x2 >= 0 && stretch.x > 0)
                {
                    x1 += stretch_face.x;
                    x2 += stretch_face.x;
                }
                if (x2 <= 0 && stretch.x < 0)
                {
                    x1 += stretch_face.x;
                    x2 += stretch_face.x;
                }
                if (z2 >= 0 && stretch.z > 0)
                {
                    z1 += stretch_face.z;
                    z2 += stretch_face.z;
                }
                if (z2 <= 0 && stretch.z < 0)
                {
                    z1 += stretch_face.z;
                    z2 += stretch_face.z;
                }

                vertices.push(x1);
                vertices.push(y1);
                vertices.push(z1);

                vertices.push(x1);
                vertices.push(y2);
                vertices.push(z1);

                vertices.push(x2);
                vertices.push(y2);
                vertices.push(z2);

                vertices.push(x2);
                vertices.push(y1);
                vertices.push(z2);

                indices.push(index_offset);
                indices.push(index_offset+1);
                indices.push(index_offset+2);

                indices.push(index_offset);
                indices.push(index_offset+2);
                indices.push(index_offset+3);
                index_offset += 4;

                // z-axis edges
                var x1 = Math.cos(ang1);
                var x2 = Math.cos(ang2);
                var y1 = Math.sin(ang1);
                var y2 = Math.sin(ang2);

                var z1 = stretch.z < 0 ? stretch_face.z : 0;
                var z2 = stretch.z > 0 ? stretch_face.z : 0;

                if (x2 >= 0 && stretch.x > 0)
                {
                    x1 += stretch_face.x;
                    x2 += stretch_face.x;
                }
                if (x2 <= 0 && stretch.x < 0)
                {
                    x1 += stretch_face.x;
                    x2 += stretch_face.x;
                }
                if (y2 >= 0 && stretch.y > 0)
                {
                    y1 += stretch_face.y;
                    y2 += stretch_face.y;
                }
                if (y2 <= 0 && stretch.y < 0)
                {
                    y1 += stretch_face.y;
                    y2 += stretch_face.y;
                }

                vertices.push(x2);
                vertices.push(y2);
                vertices.push(z1);

                vertices.push(x2);
                vertices.push(y2);
                vertices.push(z2);

                vertices.push(x1);
                vertices.push(y1);
                vertices.push(z2);

                vertices.push(x1);
                vertices.push(y1);
                vertices.push(z1);

                indices.push(index_offset);
                indices.push(index_offset+1);
                indices.push(index_offset+2);

                indices.push(index_offset);
                indices.push(index_offset+2);
                indices.push(index_offset+3);
                index_offset += 4;
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
