<script>
    export let padded = false;
</script>

<div class="content" class:padded="{padded}">
    <slot></slot>
</div>

<style>
    .content {
        box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.03), 0 1px 3px 1px rgba(0, 0, 0, 0.07), 0 2px 6px 2px rgba(0, 0, 0, 0.03);
        border-radius: 12px;
        background: white;
    }

    .padded {
        padding: 20px;
    }
</style>