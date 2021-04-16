import("../pkg/index.js")
  .then((module) => {
    const param1 = document.querySelector("#param1");
    const param2 = document.querySelector("#param2");
    const canvas = document.querySelector("canvas");

    const ctx = canvas.getContext("2d");
    const img = ctx.createImageData(canvas.width, canvas.height);

    requestAnimationFrame(redraw);
    function redraw(t) {
      const val1 = param1.value;
      const val2 = param2.value;
      module.draw_pixels(
        canvas.width,
        canvas.height,
        img.data,
        +val1,
        +val2,
        t
      );
      ctx.putImageData(img, 0, 0);
      requestAnimationFrame(redraw);
    }
  })
  .catch(console.error);
