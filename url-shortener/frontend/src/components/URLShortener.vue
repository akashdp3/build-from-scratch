<script setup>
import { ref } from 'vue';
const form = ref({ url: "" });
const isLoading = ref(false);
const error = ref(null);
const shortenedUrl = ref(null);

const handleFormSubmit = async () => {
    isLoading.value = true;
    error.value = null;
    shortenedUrl.value = null;

    try {
        const response = await fetch("http://localhost:8000/url", { method: "POST", headers: { "Content-Type": "application/json"  },  body: JSON.stringify({ url: form.value.url }) });
        const result = await response.json();
        
        shortenedUrl.value = result.shortened_url;
    } catch(e) {
        console.error("Error: Failed to shorten the url.", e);
    } finally {
        isLoading.value = false;
    }
}

const handleCopyToClipboard = () => {
    navigator.clipboard.writeText(shortenedUrl.value)
      .then(() => {
        console.log('Text successfully copied to clipboard');
      })
      .catch(err => {
        console.error('Could not copy text: ', err);
      });
}

</script>

<template>
    <form @submit.prevent="handleFormSubmit">
        <h2>URL Shortener</h2>

        <div class="url-input-field">
            <label>Shorten your URL</label>
            <input name="url" v-model="form.url"  placeholder="Enter your URL to shorten"  />
        </div>
        <button class="submit-button"  type="submit" :disabled="isLoading">
            {{isLoading ? "Shortening..." : "Submit"}}
        </button>
        
        <p class="message success" v-if="shortenedUrl">
            <span>Shortened URL: {{ shortenedUrl  }}</span>
            <button class="icon-button" aria-hidden="true" @click.prevent="handleCopyToClipboard">
                <svg width="20" height="20" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="M4 2C2.895 2 2 2.895 2 4v14h2V4h14V2H4zm4 4C6.895 6 6 6.895 6 8v12c0 1.105.895 2 2 2h12c1.105 0 2-.895 2-2V8c0-1.105-.895-2-2-2H8zm0 2h12v12H8V8z" fill="currentColor" />
                </svg>
            </button>
        </p>
        <p class="message error" v-if="error">
            <span>{{ error }}</span>
        </p>


    </form>
</template>

<style scored>
form {
    background-color: #fff;
    border-radius: 12px;
    width: 400px;
    padding: 32px;

   display: flex;
   flex-direction: column;
   gap: 16px;
}

label {
    font-size: 12px;
    line-height: 16px;
    font-weight: 600;
}

input {
    width: stretch;

    padding: 16px 12px;
    border-radius: 6px;
    border: 1px solid #ccc;
}

.url-input-field {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    gap: 8px;
}

.submit-button {
    background-color: #FFD43D;
    color: #171717;
    font-weight: bold;
}

.submit-button:hover {
    opacity: 0.8;
}


.message {
    width: stretch;
    padding: 16px 12px;
    font-size: 16px;
    line-height: 20px;
    font-weight: bold;
    border-radius: 6px;
    margin: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
}

.icon {
    display: inline-flex;
}

.success {
    background-color: lightgreen;
    border: 1px solid darkgreen;
    color: green;
}

.error {
    background-color: #FFCCCB;
    border: 1px solid darkred;
    color: red;
}

.icon-button {
    background-color: transparent;
    padding: 8px;
    border-radius: 6px;
    border-color: none;
    color: green;
}

.icon-button:hover {
    background-color: #4ac64a;
}

</style>
