<script setup>
import { onMounted } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const fetchOriginalUrl = () => {
    const shortCode = route.params.shortCode;

    const response 
}

</script>

<template> 
    <div>URL Redirect Component</div>
</template>