<script>
  export let text = "Didn't provide text";
  export let icon = undefined;
  export let description = undefined;

  import { onMount } from "svelte";

  const icons = {
    eye: "",
    light:
      '<svg viewBox="0 0 384 512"><path d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>',
  };

  let state = null;
  let box;
  onMount(() => {
    box.indeterminate = true;
  });
  function onClick(e) {
    if (state === null) {
      box.indeterminate = false;
      box.checked = true;
      state = true;
    } else if (state == true) {
      box.indeterminate = false;
      box.checked = false;
      state = false;
    } else {
      box.indeterminate = true;
      box.checked = false;
      state = null;
    }
    console.log(box.indeterminate);
  }
</script>

<div class="form-control">
  <label class="label cursor-pointer">
    <span class="label-text flex flex-row gap-1">
      {#if icon === "eye"}
        <svg class="w-4" viewBox="0 0 640 512">
          <path
            d="M150.7 92.77C195 58.27 251.8 32 320 32C400.8 32 465.5 68.84 512.6 112.6C559.4 156 590.7 207.1 605.5 243.7C608.8 251.6 608.8 260.4 605.5 268.3C592.1 300.6 565.2 346.1 525.6 386.7L630.8 469.1C641.2 477.3 643.1 492.4 634.9 502.8C626.7 513.2 611.6 515.1 601.2 506.9L9.196 42.89C-1.236 34.71-3.065 19.63 5.112 9.196C13.29-1.236 28.37-3.065 38.81 5.112L150.7 92.77zM223.1 149.5L313.4 220.3C317.6 211.8 320 202.2 320 191.1C320 180.5 316.1 169.7 311.6 160.4C314.4 160.1 317.2 159.1 320 159.1C373 159.1 416 202.1 416 255.1C416 269.7 413.1 282.7 407.1 294.5L446.6 324.7C457.7 304.3 464 280.9 464 255.1C464 176.5 399.5 111.1 320 111.1C282.7 111.1 248.6 126.2 223.1 149.5zM393.6 469.4L54.65 203.7C62.6 190.1 72.08 175.8 83.09 161.5L446.2 447.5C429.8 456.4 412.3 463.8 393.6 469.4V469.4zM34.46 268.3C31.74 261.8 31.27 254.5 33.08 247.8L329.2 479.8C326.1 479.9 323.1 480 320 480C239.2 480 174.5 443.2 127.4 399.4C80.62 355.1 49.34 304 34.46 268.3H34.46z"
          />
        </svg>
      {:else if icon === "light"}
        <svg class="w-4" viewBox="0 0 384 512">
          <path
            d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"
          />
        </svg>
      {/if}

      {text}
      {#if description}
        <div class="dropdown relative">
          <label tabindex="0" class="btn btn-circle btn-ghost btn-xs text-info">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-4 h-4 stroke-current">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </label>
          <div tabindex="0" class="card compact dropdown-content shadow bg-base-100 rounded-box w-64">
            <div class="card-body">
              <p>{description}</p>
            </div>
          </div>
        </div>
      {/if}
    </span>
    <input id="cbox" type="checkbox" on:click={onClick} class="checkbox" bind:this={box} />
  </label>
</div>
