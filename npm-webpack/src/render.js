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

        this.action_data = action_data;
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

        this.frame_index = parseInt(this.get_from_url("frame"), 10);
        // handle invalid frame index
        if (Number.isNaN(this.frame_index) || this.frame_index < 0 || this.frame_index >= this.action_data.frames.length) {
            this.frame_index = 0;
        }

        this.ecb_checkbox = document.getElementById('ecb-checkbox');
        this.ecb_checkbox.checked = this.get_bool_from_url("ecb");

        this.wireframe_checkbox = document.getElementById('wireframe-checkbox');
        this.wireframe_checkbox.checked = this.get_bool_from_url("wireframe");
        this.wireframe_toggle();

        this.run = false;
        this.ecb_material = new three.MeshBasicMaterial({ color: 0xf15c0a, transparent: true, opacity: 0.5, side: three.DoubleSide });
        this.hitbox_material = new three.MeshBasicMaterial({ color: 0xff0000, transparent: true, opacity: 0.5 });
        this.grabbox_material = new three.MeshBasicMaterial({ color: 0xff00ff, transparent: true, opacity: 0.5 });

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

    wireframe_toggle() {
        if (this.wireframe_checkbox.checked) {
            this.hurtbox_normal_material = new three.MeshBasicMaterial({ color: 0xffff00, transparent: true, wireframe: true });
            this.hurtbox_intangible_material = new three.MeshBasicMaterial({ color: 0x0000ff, transparent: true, wireframe: true });
            this.hurtbox_invincible_material = new three.MeshBasicMaterial({ color: 0x00ff00, transparent: true, wireframe: true });
        } else {
            this.hurtbox_normal_material = new three.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.4 });
            this.hurtbox_intangible_material = new three.MeshBasicMaterial({ color: 0x0000ff, transparent: true, opacity: 0.4 });
            this.hurtbox_invincible_material = new three.MeshBasicMaterial({ color: 0x00ff00, transparent: true, opacity: 0.4 });
        }
        this.setup_frame();
        this.set_bool_in_url("wireframe", this.wireframe_checkbox.checked);
    }

    ecb_toggle() {
        this.setup_frame();
        this.set_bool_in_url("ecb", this.ecb_checkbox.checked);
    }

    run_toggle() {
        if (this.run) {
            // this.animate() increments this.frame_index after calling this.setup_frame() to avoid skipping the first frame.
            // However that means frame_index is ahead of where it should be.
            // So before this.stop() we decrement this.frame_index back to where it was.
            this.frame_index -= 1;
            if (this.frame_index == -1) {
                this.frame_index = this.action_data.frames.length - 1;
            }

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

        this.set_in_url("frame", Math.max(0, Math.min(this.action_data.frames.length-1, this.frame_index)));
    }

    previous_frame() {
        this.frame_index -= 1;
        if (this.frame_index == -1) {
            this.frame_index = this.action_data.frames.length - 1;
        }
        this.stop();
        this.setup_frame();
    }

    next_frame() {
        this.frame_index += 1;
        if (this.frame_index >= this.action_data.frames.length) {
            this.frame_index = 0;
        }
        this.stop();
        this.setup_frame();
    }

    set_frame(index) {
        this.frame_index = index - 1;
        this.stop();
        this.setup_frame();
    }

    face_left() {
        this.camera.position.set(60, 8, 0);
        this.controls.update();
    }

    face_right() {
        this.camera.position.set(-60, 8, 0);
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

        // generate ecb
        if (this.ecb_checkbox.checked) {
            const mid_y = (frame.ecb.top + frame.ecb.bottom) / 2.0;
            const vertices = [
                0, frame.ecb.top,    0,
                0, mid_y,            frame.ecb.left,
                0, mid_y,            frame.ecb.right,
                0, frame.ecb.bottom, 0,
            ];

            const indices = [
                0, 1, 2,
                1, 2, 3,
            ];

            const geometry = new three.BufferGeometry();
            geometry.addAttribute('position', new three.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            this.scene.add(new three.Mesh(geometry, this.ecb_material));
        }

        // generate hitboxes
        for (let hit_box of frame.hit_boxes) {
            // hit/grab box specific logic
            var material = this.hitbox_material;
            if (hit_box.next_values.Grab != null) {
                material = this.grabbox_material;
            }
            else if(hit_box.next_values.Hit != null) {
                if (!hit_box.next_values.Hit.can_hit_multiplayer_characters) {
                    // only display hitboxes that are used in regular matches
                    continue;
                }
            }

            var prev_distance = 0;
            var prev = null;
            const next = new three.Vector3(hit_box.next_pos.x, hit_box.next_pos.y, hit_box.next_pos.z);
            if (hit_box.prev_pos != null) {
                prev = new three.Vector3(hit_box.prev_pos.x, hit_box.prev_pos.y, hit_box.prev_pos.z);
                prev_distance = next.distanceTo(prev);
            }

            const vertices = [];
            const indices = [];
            const widthSegments = 23;
            const heightSegments = 17;
            const grid = []
            var index_offset = 0;
            // modified UV sphere generation from:
            // https://github.com/mrdoob/three.js/blob/4ca3860851d0cd33535afe801a1aa856da277f3a/src/geometries/SphereGeometry.js
            for (var iy = 0; iy <= heightSegments; iy++) {
                var verticesRow = [];
                var v = iy / heightSegments;

                for (var ix = 0; ix <= widthSegments; ix++) {
                    var u = ix / widthSegments;
                    var y_offset = 0;
                    if (prev != null) {
                        if (v >= 0 && v <= 0.5) {
                            y_offset += prev_distance;
                        }
                    }

                    const sin_v_pi = Math.sin(v * Math.PI);
                    vertices.push(hit_box.next_size * Math.cos(u * Math.PI * 2) * sin_v_pi);
                    vertices.push(hit_box.next_size * Math.cos(v * Math.PI) + y_offset);
                    vertices.push(hit_box.next_size * Math.sin(u * Math.PI * 2) * sin_v_pi);

                    verticesRow.push(index_offset++);
                }
                grid.push(verticesRow);
            }

            for (var iy = 0; iy < heightSegments; iy++) {
                for (var ix = 0; ix < widthSegments; ix++) {
                    var a = grid[iy][ix + 1];
                    var b = grid[iy][ix];
                    var c = grid[iy + 1][ix];
                    var d = grid[iy + 1][ix + 1];

                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }

            const geometry = new three.BufferGeometry();
            geometry.addAttribute('position', new three.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            const hit_box_mesh = new three.Mesh(geometry, material);

            const rotation = new three.Quaternion();
            if (prev != null) {
                const diff = prev.clone();
                diff.sub(next);
                diff.normalize();
                rotation.setFromUnitVectors(new three.Vector3(0, 1, 0), diff);
            }

            const transform = new three.Matrix4();
            transform.compose(next, rotation, new three.Vector3(1, 1, 1));

            hit_box_mesh.matrixAutoUpdate = false;
            hit_box_mesh.matrix.copy(transform);

            this.scene.add(hit_box_mesh);
        }

        // generate hurtboxes
        for (let hurt_box of frame.hurt_boxes) {
            const bm = hurt_box.bone_matrix;
            const bone_matrix = new three.Matrix4();
            bone_matrix.set(
                bm.x.x, bm.y.x, bm.z.x, bm.w.x,
                bm.x.y, bm.y.y, bm.z.y, bm.w.y,
                bm.x.z, bm.y.z, bm.z.z, bm.w.z,
                bm.x.w, bm.y.w, bm.z.w, bm.w.w
            );
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
                const verticesRow = [];
                const v = iy / heightSegments;

                for (var ix = 0; ix <= widthSegments; ix++) {
                    const u = ix / widthSegments;

                    // The x, y and z stretch values, split the sphere in half, across its dimension.
                    // This can result in 8 individual sphere corners.
                    const corner_offset = new three.Vector3();
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
                    const a = grid[iy][(ix + 1) % widthSegments];
                    const b = grid[iy][ix];
                    const c = grid[iy + 1][ix];
                    const d = grid[iy + 1][(ix + 1) % widthSegments];

                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }

            const geometry = new three.BufferGeometry();
            geometry.addAttribute('position', new three.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            var material = this.hurtbox_normal_material;
            if (hurt_box.state == "IntangibleFlashing" || hurt_box.state == "IntangibleNoFlashing" || hurt_box.state == "IntangibleQuickFlashing") {
                material = this.hurtbox_intangible_material;
            }
            else if (hurt_box.state == "Invincible") {
                material = this.hurtbox_invincible_material;
            }

            const cube = new three.Mesh(geometry, material);

            const transform_translation = new three.Matrix4();
            transform_translation.makeTranslation(
                hurt_box.hurt_box.offset.x / (bone_scale.x * radius),
                hurt_box.hurt_box.offset.y / (bone_scale.y * radius),
                hurt_box.hurt_box.offset.z / (bone_scale.z * radius)
            );

            const transform_scale = new three.Matrix4();
            transform_scale.makeScale(radius, radius, radius);

            const transform = new three.Matrix4();
            transform.copy(bone_matrix);
            transform.multiply(transform_scale);
            transform.multiply(transform_translation);

            cube.matrixAutoUpdate = false;
            cube.matrix.copy(transform);
            this.scene.add(cube);
        }

        // update frame select
        for (var button of document.querySelectorAll('.frame-button')) {
            button.classList.remove('current-frame-button');
            if (parseInt(button.innerHTML, 10) - 1 == this.frame_index) {
                button.classList.add('current-frame-button');
            }
        }
    }

    animate() {
        if (this.run) {
            // this.frame_index needs to be incremented after this.setup_frame() to avoid skipping the first frame
            this.setup_frame();
            this.frame_index += 1;
            if (this.frame_index >= this.action_data.frames.length) {
                this.frame_index = 0;
            }
        }

        this.renderer.render(this.scene, this.camera);
        requestAnimationFrame(() => this.animate());
    }

    set_in_url(name, data) {
        var url = new URL(location);
        url.searchParams.set(name, data);
        window.history.replaceState({}, "", url);
    }

    set_bool_in_url(name, data) {
        var url = new URL(location);
        if (data) {
            url.searchParams.set(name, data);
        } else {
            url.searchParams.delete(name);
        }
        window.history.replaceState({}, "", url);
    }

    get_from_url(name) {
        var url = new URL(location);
        return url.searchParams.get(name);
    }

    get_bool_from_url(name) {
        var url = new URL(location);
        return url.searchParams.get(name) === "true";
    }
}
