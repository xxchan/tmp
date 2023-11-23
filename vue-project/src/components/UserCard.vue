<script>
export default {
    props: {
        user: {
            type: Object,
            required: true,
        }
    },
    emites: ['change-name'],
}
</script>

<template>
    <h1>User: {{ user.name }}</h1>
    <p>age: {{ user.age }}</p>
    <button @click="$emit('change-name')">change name</button>
</template>