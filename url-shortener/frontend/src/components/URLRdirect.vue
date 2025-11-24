<script setup>
import { onMounted } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const redirectToOriginal = () => {
    const shortCode = route.params.shortCode;
    window.location.replace(`http://localhost:8000/url?key=${encodeURIComponent(shortCode)}`);
}

onMounted(redirectToOriginal);
</script>

<template> 
    <div>Redirecting...</div>
</template>