<script>

export default {
    data() {
        return {
            count: 0,
            // title: "Counter",
            incBy: 2,
        }
    },
    computed: {
        // dependencies are automatically tracked!
        title() {
            console.log("title computed", this.count)
            return this.count > 20 ? "Large Counter" : "Counter" + "Factor: " + this.factor
        },
        factor() {
            console.log("factor computed", this.count)
            return this.count % 2
        }
    },
    methods: {
        increment() {
            this.count += this.incBy
        },
        favorite(character) {
            this.favoriteCharacters.push(character)
        },
    },

    watch: {
        // // replaced with computed property for better performance and dependency tracking
        // // watch is good for side effects 

        // count(value) {
        //   console.log(value)
        //   if (value > 20) {
        //     this.title = "Large Counter"
        //   }
        // }
    }
}
</script>

<template>
    <h1>{{ title }}</h1>
    <p>{{ count }}</p>
    <p v-if="count % 2 === 0"> Even</p>
    <p v-else> Odd</p>
    <button v-on:click="increment">Increment</button>
    <div>
        <label for="incBy">incBy</label>
        <input type="number" name="incBy" id="incBy" v-model="incBy">
    </div>
</template>