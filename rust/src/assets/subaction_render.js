class OrbitControls extends THREE.EventDispatcher {
  constructor (object, domElement) {
    super();

    this.object = object;
    this.domElement = (domElement !== undefined) ? domElement : document;

    // Set to false to disable this control
    this.enabled = true;

    // "target" sets the location of focus, where the object orbits around
    this.target = new THREE.Vector3();

    // How far you can dolly in and out ( PerspectiveCamera only )
    this.minDistance = 0;
    this.maxDistance = Infinity;

    // How far you can zoom in and out ( OrthographicCamera only )
    this.minZoom = 0;
    this.maxZoom = Infinity;

    // How far you can orbit vertically, upper and lower limits.
    // Range is 0 to Math.PI radians.
    this.minPolarAngle = 0; // radians
    this.maxPolarAngle = Math.PI; // radians

    // How far you can orbit horizontally, upper and lower limits.
    // If set, must be a sub-interval of the interval [ - Math.PI, Math.PI ].
    this.minAzimuthAngle = -Infinity; // radians
    this.maxAzimuthAngle = Infinity; // radians

    // Set to true to enable damping (inertia)
    // If damping is enabled, you must call controls.update() in your animation loop
    this.enableDamping = false;
    this.dampingFactor = 0.25;

    // This option actually enables dollying in and out; left as "zoom" for backwards compatibility.
    // Set to false to disable zooming
    this.enableZoom = true;
    this.zoomSpeed = 1.0;

    // Set to false to disable rotating
    this.enableRotate = true;
    this.rotateSpeed = 1.0;

    // Set to false to disable panning
    this.enablePan = true;
    this.keyPanSpeed = 7.0; // pixels moved per arrow key push

    // Set to true to automatically rotate around the target
    // If auto-rotate is enabled, you must call controls.update() in your animation loop
    this.autoRotate = false;
    this.autoRotateSpeed = 2.0; // 30 seconds per round when fps is 60

    // Set to false to disable use of the keys
    this.enableKeys = true;

    // The four arrow keys
    this.keys = {
      LEFT: 37,
      UP: 38,
      RIGHT: 39,
      BOTTOM: 40
    };

    // Mouse buttons
    this.mouseButtons = {
      ORBIT: THREE.MOUSE.LEFT,
      ZOOM: THREE.MOUSE.MIDDLE,
      PAN: THREE.MOUSE.RIGHT
    };

    // for reset
    this.target0 = this.target.clone();
    this.position0 = this.object.position.clone();
    this.zoom0 = this.object.zoom;

    //
    // public methods
    //

    this.getPolarAngle = () => spherical.phi;
    this.getAzimuthalAngle = () => spherical.theta;

    // this method is exposed, but perhaps it would be better if we can make it private...
    this.update = (function () {
      var offset = new THREE.Vector3();

      // so camera.up is the orbit axis
      var quat = new THREE.Quaternion().setFromUnitVectors(object.up, new THREE.Vector3(0, 1, 0));
      var quatInverse = quat.clone().inverse();

      var lastPosition = new THREE.Vector3();
      var lastQuaternion = new THREE.Quaternion();

      return function update () {
        var position = scope.object.position;

        offset.copy(position).sub(scope.target);

        // rotate offset to "y-axis-is-up" space
        offset.applyQuaternion(quat);

        // angle from z-axis around y-axis
        spherical.setFromVector3(offset);

        if (scope.autoRotate && state === STATE.NONE) {
          rotateLeft(getAutoRotationAngle());
        }

        spherical.theta += sphericalDelta.theta;
        spherical.phi += sphericalDelta.phi;

        // restrict theta to be between desired limits
        spherical.theta = Math.max(scope.minAzimuthAngle, Math.min(scope.maxAzimuthAngle, spherical.theta));

        // restrict phi to be between desired limits
        spherical.phi = Math.max(scope.minPolarAngle, Math.min(scope.maxPolarAngle, spherical.phi));
        spherical.makeSafe();
        spherical.radius *= scale;

        // restrict radius to be between desired limits
        spherical.radius = Math.max(scope.minDistance, Math.min(scope.maxDistance, spherical.radius));

        // move target to panned location
        scope.target.add(panOffset);

        offset.setFromSpherical(spherical);

        // rotate offset back to "camera-up-vector-is-up" space
        offset.applyQuaternion(quatInverse);

        position.copy(scope.target).add(offset);

        scope.object.lookAt(scope.target);

        if (scope.enableDamping === true) {
          sphericalDelta.theta *= (1 - scope.dampingFactor);
          sphericalDelta.phi *= (1 - scope.dampingFactor);
        } else {
          sphericalDelta.set(0, 0, 0);
        }

        scale = 1;
        panOffset.set(0, 0, 0);

        // update condition is:
        // min(camera displacement, camera rotation in radians)^2 > EPS
        // using small-angle approximation cos(x/2) = 1 - x^2 / 8

        if (zoomChanged ||
          lastPosition.distanceToSquared(scope.object.position) > EPS ||
          8 * (1 - lastQuaternion.dot(scope.object.quaternion)) > EPS) {
          scope.dispatchEvent(changeEvent);

          lastPosition.copy(scope.object.position);
          lastQuaternion.copy(scope.object.quaternion);
          zoomChanged = false;

          return true
        }

        return false
      }
    }());

    this.dispose = function () {
      scope.domElement.removeEventListener('contextmenu', onContextMenu, false);
      scope.domElement.removeEventListener('mousedown', onMouseDown, false);
      scope.domElement.removeEventListener('wheel', onMouseWheel, false);

      scope.domElement.removeEventListener('touchstart', onTouchStart, false);
      scope.domElement.removeEventListener('touchend', onTouchEnd, false);
      scope.domElement.removeEventListener('touchmove', onTouchMove, false);

      document.removeEventListener('mousemove', onMouseMove, false);
      document.removeEventListener('mouseup', onMouseUp, false);

      window.removeEventListener('keydown', onKeyDown, false);
    };

    //
    // internals
    //

    var scope = this;
    var changeEvent = {
      type: 'change'
    };
    var startEvent = {
      type: 'start'
    };
    var endEvent = {
      type: 'end'
    };

    var STATE = {
      NONE: -1,
      ROTATE: 0,
      DOLLY: 1,
      PAN: 2,
      TOUCH_ROTATE: 3,
      TOUCH_DOLLY: 4,
      TOUCH_PAN: 5
    };

    var state = STATE.NONE;

    var EPS = 0.000001;

    // current position in spherical coordinates
    var spherical = new THREE.Spherical();
    var sphericalDelta = new THREE.Spherical();

    var scale = 1;
    var panOffset = new THREE.Vector3();
    var zoomChanged = false;

    var rotateStart = new THREE.Vector2();
    var rotateEnd = new THREE.Vector2();
    var rotateDelta = new THREE.Vector2();

    var panStart = new THREE.Vector2();
    var panEnd = new THREE.Vector2();
    var panDelta = new THREE.Vector2();

    var dollyStart = new THREE.Vector2();
    var dollyEnd = new THREE.Vector2();
    var dollyDelta = new THREE.Vector2();

    function getAutoRotationAngle () {
      return 2 * Math.PI / 60 / 60 * scope.autoRotateSpeed
    }

    function getZoomScale () {
      return Math.pow(0.95, scope.zoomSpeed)
    }

    function rotateLeft (angle) {
      sphericalDelta.theta -= angle;
    }

    function rotateUp (angle) {
      sphericalDelta.phi -= angle;
    }

    var panLeft = (function () {
      var v = new THREE.Vector3();

      return function panLeft (distance, objectMatrix) {
        v.setFromMatrixColumn(objectMatrix, 0); // get X column of objectMatrix
        v.multiplyScalar(-distance);

        panOffset.add(v);
      }
    }());

    var panUp = (function () {
      var v = new THREE.Vector3();

      return function panUp (distance, objectMatrix) {
        v.setFromMatrixColumn(objectMatrix, 1); // get Y column of objectMatrix
        v.multiplyScalar(distance);

        panOffset.add(v);
      }
    }());

    // deltaX and deltaY are in pixels; right and down are positive
    var pan = (function () {
      var offset = new THREE.Vector3();

      return function pan (deltaX, deltaY) {
        var element = scope.domElement === document ? scope.domElement.body : scope.domElement;

        if (Object.getPrototypeOf(scope.object).isPerspectiveCamera) {
          // perspective
          var position = scope.object.position;
          offset.copy(position).sub(scope.target);
          var targetDistance = offset.length();

          // half of the fov is center to top of screen
          targetDistance *= Math.tan((scope.object.fov / 2) * Math.PI / 180.0);

          // we actually don't use screenWidth, since perspective camera is fixed to screen height
          panLeft(2 * deltaX * targetDistance / element.clientHeight, scope.object.matrix);
          panUp(2 * deltaY * targetDistance / element.clientHeight, scope.object.matrix);
        } else if (Object.getPrototypeOf(scope.object).isOrthographicCamera) {
          // orthographic
          panLeft(deltaX * (scope.object.right - scope.object.left) / scope.object.zoom / element.clientWidth, scope.object.matrix);
          panUp(deltaY * (scope.object.top - scope.object.bottom) / scope.object.zoom / element.clientHeight, scope.object.matrix);
        } else {
          // camera neither orthographic nor perspective
          console.warn('WARNING: OrbitControls.js encountered an unknown camera type - pan disabled.');
          scope.enablePan = false;
        }
      }
    }());

    function dollyIn (dollyScale) {
      if (Object.getPrototypeOf(scope.object).isPerspectiveCamera) {
        scale /= dollyScale;
      } else if (Object.getPrototypeOf(scope.object).isOrthographicCamera) {
        scope.object.zoom = Math.max(scope.minZoom, Math.min(scope.maxZoom, scope.object.zoom * dollyScale));
        scope.object.updateProjectionMatrix();
        zoomChanged = true;
      } else {
        console.warn('WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled.');
        scope.enableZoom = false;
      }
    }

    function dollyOut (dollyScale) {
      if (Object.getPrototypeOf(scope.object).isPerspectiveCamera) {
        scale *= dollyScale;
      } else if (Object.getPrototypeOf(scope.object).isOrthographicCamera) {
        scope.object.zoom = Math.max(scope.minZoom, Math.min(scope.maxZoom, scope.object.zoom / dollyScale));
        scope.object.updateProjectionMatrix();
        zoomChanged = true;
      } else {
        console.warn('WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled.');
        scope.enableZoom = false;
      }
    }

    //
    // event callbacks - update the object state
    //

    function handleMouseDownRotate (event) {
      rotateStart.set(event.clientX, event.clientY);
    }

    function handleMouseDownDolly (event) {
      dollyStart.set(event.clientX, event.clientY);
    }

    function handleMouseDownPan (event) {
      panStart.set(event.clientX, event.clientY);
    }

    function handleMouseMoveRotate (event) {
      rotateEnd.set(event.clientX, event.clientY);
      rotateDelta.subVectors(rotateEnd, rotateStart);
      var element = scope.domElement === document ? scope.domElement.body : scope.domElement;

      // rotating across whole screen goes 360 degrees around
      rotateLeft(2 * Math.PI * rotateDelta.x / element.clientWidth * scope.rotateSpeed);

      // rotating up and down along whole screen attempts to go 360, but limited to 180
      rotateUp(2 * Math.PI * rotateDelta.y / element.clientHeight * scope.rotateSpeed);
      rotateStart.copy(rotateEnd);
      scope.update();
    }

    function handleMouseMoveDolly (event) {
      dollyEnd.set(event.clientX, event.clientY);
      dollyDelta.subVectors(dollyEnd, dollyStart);

      if (dollyDelta.y > 0) {
        dollyIn(getZoomScale());
      } else if (dollyDelta.y < 0) {
        dollyOut(getZoomScale());
      }

      dollyStart.copy(dollyEnd);
      scope.update();
    }

    function handleMouseMovePan (event) {
      panEnd.set(event.clientX, event.clientY);
      panDelta.subVectors(panEnd, panStart);
      pan(panDelta.x, panDelta.y);
      panStart.copy(panEnd);
      scope.update();
    }

    function handleMouseWheel (event) {
      if (event.deltaY < 0) {
        dollyOut(getZoomScale());
      } else if (event.deltaY > 0) {
        dollyIn(getZoomScale());
      }

      scope.update();
    }

    function handleKeyDown (event) {
      switch (event.keyCode) {
        case scope.keys.UP:
          pan(0, scope.keyPanSpeed);
          scope.update();
          break

        case scope.keys.BOTTOM:
          pan(0, -scope.keyPanSpeed);
          scope.update();
          break

        case scope.keys.LEFT:
          pan(scope.keyPanSpeed, 0);
          scope.update();
          break

        case scope.keys.RIGHT:
          pan(-scope.keyPanSpeed, 0);
          scope.update();
          break
      }
    }

    function handleTouchStartRotate (event) {
      rotateStart.set(event.touches[0].pageX, event.touches[0].pageY);
    }

    function handleTouchStartDolly (event) {
      var dx = event.touches[0].pageX - event.touches[1].pageX;
      var dy = event.touches[0].pageY - event.touches[1].pageY;

      var distance = Math.sqrt(dx * dx + dy * dy);

      dollyStart.set(0, distance);
    }

    function handleTouchStartPan (event) {
      panStart.set(event.touches[0].pageX, event.touches[0].pageY);
    }

    function handleTouchMoveRotate (event) {
      rotateEnd.set(event.touches[0].pageX, event.touches[0].pageY);
      rotateDelta.subVectors(rotateEnd, rotateStart);

      var element = scope.domElement === document ? scope.domElement.body : scope.domElement;

      // rotating across whole screen goes 360 degrees around
      rotateLeft(2 * Math.PI * rotateDelta.x / element.clientWidth * scope.rotateSpeed);

      // rotating up and down along whole screen attempts to go 360, but limited to 180
      rotateUp(2 * Math.PI * rotateDelta.y / element.clientHeight * scope.rotateSpeed);

      rotateStart.copy(rotateEnd);

      scope.update();
    }

    function handleTouchMoveDolly (event) {
      var dx = event.touches[0].pageX - event.touches[1].pageX;
      var dy = event.touches[0].pageY - event.touches[1].pageY;

      var distance = Math.sqrt(dx * dx + dy * dy);

      dollyEnd.set(0, distance);

      dollyDelta.subVectors(dollyEnd, dollyStart);

      if (dollyDelta.y > 0) {
        dollyOut(getZoomScale());
      } else if (dollyDelta.y < 0) {
        dollyIn(getZoomScale());
      }

      dollyStart.copy(dollyEnd);

      scope.update();
    }

    function handleTouchMovePan (event) {
      panEnd.set(event.touches[0].pageX, event.touches[0].pageY);
      panDelta.subVectors(panEnd, panStart);
      pan(panDelta.x, panDelta.y);
      panStart.copy(panEnd);
      scope.update();
    }

    //
    // event handlers - FSM: listen for events and reset state
    //

    function onMouseDown (event) {
      if (scope.enabled === false) return

      event.preventDefault();

      if (event.button === scope.mouseButtons.ORBIT) {
        if (scope.enableRotate === false) return

        handleMouseDownRotate(event);

        state = STATE.ROTATE;
      } else if (event.button === scope.mouseButtons.ZOOM) {
        if (scope.enableZoom === false) return

        handleMouseDownDolly(event);

        state = STATE.DOLLY;
      } else if (event.button === scope.mouseButtons.PAN) {
        if (scope.enablePan === false) return

        handleMouseDownPan(event);

        state = STATE.PAN;
      }

      if (state !== STATE.NONE) {
        document.addEventListener('mousemove', onMouseMove, false);
        document.addEventListener('mouseup', onMouseUp, false);

        scope.dispatchEvent(startEvent);
      }
    }

    function onMouseMove (event) {
      if (scope.enabled === false) return

      event.preventDefault();

      if (state === STATE.ROTATE) {
        if (scope.enableRotate === false) return

        handleMouseMoveRotate(event);
      } else if (state === STATE.DOLLY) {
        if (scope.enableZoom === false) return

        handleMouseMoveDolly(event);
      } else if (state === STATE.PAN) {
        if (scope.enablePan === false) return

        handleMouseMovePan(event);
      }
    }

    function onMouseUp (event) {
      if (scope.enabled === false) return

      document.removeEventListener('mousemove', onMouseMove, false);
      document.removeEventListener('mouseup', onMouseUp, false);

      scope.dispatchEvent(endEvent);

      state = STATE.NONE;
    }

    function onMouseWheel (event) {
      if (scope.enabled === false || scope.enableZoom === false || (state !== STATE.NONE && state !== STATE.ROTATE)) return

      event.preventDefault();
      event.stopPropagation();
      handleMouseWheel(event);

      scope.dispatchEvent(startEvent); // not sure why these are here...
      scope.dispatchEvent(endEvent);
    }

    function onKeyDown (event) {
      if (scope.enabled === false || scope.enableKeys === false || scope.enablePan === false) return
      handleKeyDown(event);
    }

    function onTouchStart (event) {
      if (scope.enabled === false) return

      switch (event.touches.length) {
        case 1: // one-fingered touch: rotate
          if (scope.enableRotate === false) return
          handleTouchStartRotate(event);
          state = STATE.TOUCH_ROTATE;

          break

        case 2: // two-fingered touch: dolly

          if (scope.enableZoom === false) return
          handleTouchStartDolly(event);
          state = STATE.TOUCH_DOLLY;

          break

        case 3: // three-fingered touch: pan

          if (scope.enablePan === false) return
          handleTouchStartPan(event);
          state = STATE.TOUCH_PAN;
          break

        default:

          state = STATE.NONE;
      }

      if (state !== STATE.NONE) {
        scope.dispatchEvent(startEvent);
      }
    }

    function onTouchMove (event) {
      if (scope.enabled === false) return

      event.preventDefault();
      event.stopPropagation();

      switch (event.touches.length) {
        case 1: // one-fingered touch: rotate

          if (scope.enableRotate === false) return
          if (state !== STATE.TOUCH_ROTATE) return // is this needed?...
          handleTouchMoveRotate(event);
          break

        case 2: // two-fingered touch: dolly

          if (scope.enableZoom === false) return
          if (state !== STATE.TOUCH_DOLLY) return // is this needed?...
          handleTouchMoveDolly(event);
          break

        case 3: // three-fingered touch: pan

          if (scope.enablePan === false) return
          if (state !== STATE.TOUCH_PAN) return // is this needed?...
          handleTouchMovePan(event);
          break

        default:
          state = STATE.NONE;
      }
    }

    function onTouchEnd (event) {
      if (scope.enabled === false) return
      scope.dispatchEvent(endEvent);
      state = STATE.NONE;
    }

    function onContextMenu (event) {
      event.preventDefault();
    }

    scope.domElement.addEventListener('contextmenu', onContextMenu, false);
    scope.domElement.addEventListener('mousedown', onMouseDown, false);
    scope.domElement.addEventListener('wheel', onMouseWheel, false);
    scope.domElement.addEventListener('touchstart', onTouchStart, false);
    scope.domElement.addEventListener('touchend', onTouchEnd, false);
    scope.domElement.addEventListener('touchmove', onTouchMove, false);

    window.addEventListener('keydown', onKeyDown, false);

    // force an update at start
    this.update();
  };

  get center () {
    console.warn('OrbitControls: .center has been renamed to .target');
    return this.target
  }
}

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

class FighterRender {
    constructor(subaction_data) {
        const render_div = document.getElementById('fighter-render');

        this.subaction_data = subaction_data;
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(40, 1, 1.0, 1000);
        this.controls = new OrbitControls(this.camera, render_div);
        this.controls.target.set(0, 8, 0);
        this.controls.update();
        this.face_right();

        this.renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
        this.renderer.setClearColor(0xFFFFFF, 0);
        render_div.appendChild(this.renderer.domElement);

        this.window_resize();
        window.addEventListener('resize', () => this.window_resize(), false);

        this.frame_index = parseInt(this.get_from_url("frame"), 10);
        // handle invalid frame index
        if (Number.isNaN(this.frame_index) || this.frame_index < 0 || this.frame_index >= this.subaction_data.frames.length) {
            this.frame_index = 0;
        }

        this.ecb_checkbox = document.getElementById('ecb-checkbox');
        this.ecb_checkbox.checked = this.get_bool_from_url("ecb");

        this.wireframe_checkbox = document.getElementById('wireframe-checkbox');
        this.wireframe_checkbox.checked = this.get_bool_from_url("wireframe");
        this.wireframe_toggle();

        this.run = false;
        this.ecb_material = new THREE.MeshBasicMaterial({ color: 0xf15c0a, transparent: true, opacity: 0.5, side: THREE.DoubleSide });
        this.hitbox_material = new THREE.MeshBasicMaterial({ color: 0xff0000, transparent: true, opacity: 0.5 });
        this.grabbox_material = new THREE.MeshBasicMaterial({ color: 0xff00ff, transparent: true, opacity: 0.5 });

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
            this.hurtbox_normal_material = new THREE.MeshBasicMaterial({ color: 0xffff00, transparent: true, wireframe: true });
            this.hurtbox_intangible_material = new THREE.MeshBasicMaterial({ color: 0x0000ff, transparent: true, wireframe: true });
            this.hurtbox_invincible_material = new THREE.MeshBasicMaterial({ color: 0x00ff00, transparent: true, wireframe: true });
        } else {
            this.hurtbox_normal_material = new THREE.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.4 });
            this.hurtbox_intangible_material = new THREE.MeshBasicMaterial({ color: 0x0000ff, transparent: true, opacity: 0.4 });
            this.hurtbox_invincible_material = new THREE.MeshBasicMaterial({ color: 0x00ff00, transparent: true, opacity: 0.4 });
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
                this.frame_index = this.subaction_data.frames.length - 1;
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

        this.set_in_url("frame", Math.max(0, Math.min(this.subaction_data.frames.length-1, this.frame_index)));
    }

    previous_frame() {
        this.frame_index -= 1;
        if (this.frame_index == -1) {
            this.frame_index = this.subaction_data.frames.length - 1;
        }
        this.stop();
        this.setup_frame();
    }

    next_frame() {
        this.frame_index += 1;
        if (this.frame_index >= this.subaction_data.frames.length) {
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
        const frame = this.subaction_data.frames[this.frame_index];

        // clear all objects from previous frame
        while (this.scene.children.length) {
            const child = this.scene.children[0];
            this.scene.remove(child);
            child.geometry.dispose();
        }

        const transform_translation_frame = new THREE.Matrix4();
        transform_translation_frame.makeTranslation(
            0.0,
            frame.y_pos,
            frame.x_pos,
        );

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

            const geometry = new THREE.BufferGeometry();
            geometry.addAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            const mesh = new THREE.Mesh(geometry, this.ecb_material);
            mesh.position.z = frame.x_pos;
            mesh.position.y = frame.y_pos;
            this.scene.add(mesh);
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
            const next = new THREE.Vector3(hit_box.next_pos.x, hit_box.next_pos.y + frame.y_pos, hit_box.next_pos.z + frame.x_pos);
            if (hit_box.prev_pos != null) {
                prev = new THREE.Vector3(hit_box.prev_pos.x, hit_box.prev_pos.y + frame.y_pos, hit_box.prev_pos.z + frame.x_pos);
                prev_distance = next.distanceTo(prev);
            }

            const vertices = [];
            const indices = [];
            const widthSegments = 23;
            const heightSegments = 17;
            const grid = []
            var index_offset = 0;
            // modified UV sphere generation from:
            // https://github.com/mrdoob/THREE.js/blob/4ca3860851d0cd33535afe801a1aa856da277f3a/src/geometries/SphereGeometry.js
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

            const geometry = new THREE.BufferGeometry();
            geometry.addAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            const hit_box_mesh = new THREE.Mesh(geometry, material);

            const rotation = new THREE.Quaternion();
            if (prev != null) {
                const diff = prev.clone();
                diff.sub(next);
                diff.normalize();
                rotation.setFromUnitVectors(new THREE.Vector3(0, 1, 0), diff);
            }

            const transform = new THREE.Matrix4();
            transform.compose(next, rotation, new THREE.Vector3(1, 1, 1));

            hit_box_mesh.matrixAutoUpdate = false;
            hit_box_mesh.matrix.copy(transform);

            this.scene.add(hit_box_mesh);
        }

        // generate hurtboxes
        for (let hurt_box of frame.hurt_boxes) {
            const bm = hurt_box.bone_matrix;
            const bone_matrix = new THREE.Matrix4();
            bone_matrix.set(
                bm.x.x, bm.y.x, bm.z.x, bm.w.x,
                bm.x.y, bm.y.y, bm.z.y, bm.w.y,
                bm.x.z, bm.y.z, bm.z.z, bm.w.z,
                bm.x.w, bm.y.w, bm.z.w, bm.w.w
            );
            const bone_scale = new THREE.Vector3();
            bone_scale.setFromMatrixScale(bone_matrix);

            const radius = hurt_box.hurt_box.radius;
            var stretch = hurt_box.hurt_box.stretch;
            var offset = hurt_box.hurt_box.offset;

            stretch = new THREE.Vector3(stretch.x, stretch.y, stretch.z);
            offset = new THREE.Vector3(offset.x, offset.y, offset.z);

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
            // https://github.com/mrdoob/THREE.js/blob/4ca3860851d0cd33535afe801a1aa856da277f3a/src/geometries/SphereGeometry.js
            for (var iy = 0; iy <= heightSegments; iy++) {
                const verticesRow = [];
                const v = iy / heightSegments;

                for (var ix = 0; ix <= widthSegments; ix++) {
                    const u = ix / widthSegments;

                    // The x, y and z stretch values, split the sphere in half, across its dimension.
                    // This can result in 8 individual sphere corners.
                    const corner_offset = new THREE.Vector3();
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

            const geometry = new THREE.BufferGeometry();
            geometry.addAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
            geometry.setIndex(indices);

            var material = this.hurtbox_normal_material;
            if (hurt_box.state == "IntangibleFlashing" || hurt_box.state == "IntangibleNoFlashing" || hurt_box.state == "IntangibleQuickFlashing") {
                material = this.hurtbox_intangible_material;
            }
            else if (hurt_box.state == "Invincible") {
                material = this.hurtbox_invincible_material;
            }

            const cube = new THREE.Mesh(geometry, material);

            const transform_translation = new THREE.Matrix4();
            transform_translation.makeTranslation(
                hurt_box.hurt_box.offset.x / (bone_scale.x * radius),
                hurt_box.hurt_box.offset.y / (bone_scale.y * radius),
                hurt_box.hurt_box.offset.z / (bone_scale.z * radius)
            );

            const transform_scale = new THREE.Matrix4();
            transform_scale.makeScale(radius, radius, radius);

            const transform = new THREE.Matrix4();
            transform.copy(transform_translation_frame);
            transform.multiply(bone_matrix);
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
            if (this.frame_index >= this.subaction_data.frames.length) {
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

window.fighter_render = new FighterRender(fighter_subaction_data);
