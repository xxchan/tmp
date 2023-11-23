<script>
import BaseCounter from './components/BaseCounter.vue'
import UserCard from './components/UserCard.vue'

export default {
  components: {
    BaseCounter,
    UserCard
  },

  data() {
    return {
      userData: {
        name: "xx",
        age: 20,
      },

      listOfNumbers: [5, 4, 3, 2, 1],
      characters: [
        { name: "a" },
        { name: "b" }
      ],
      favoriteCharacters: [],
      newCharacter: { name: '' }
    }
  },

  methods: {
    addCharacter() {
      this.characters.push(this.newCharacter)
      this.newCharacter = { name: '' }
    },
    changeName(newName) {
      this.userData.name = newName
    }
  }


}
</script>

<template>
  <BaseCounter />

  <hr>

  <UserCard :user=userData @change-name="changeName('haha')" />

  <hr>
  <ul v-for="(character, idx) in characters">
    <li>{{ idx }}: {{ character }}
      <button v-on:click="favorite(character)">favorite 🌟</button>
    </li>
  </ul>
  <p>favorites: {{ favoriteCharacters }}</p>
  <hr>

  <div>
    <pre>{{ newCharacter }}</pre>
    <label for="newCharacter">name</label>
    <input type="text" name="newCharacter" id="newCharacter" v-model="newCharacter.name" @keyup.enter="addCharacter">
    <button @click="addCharacter">Add</button>
  </div>


  <hr>
  <ul v-for="(item, idx) in listOfNumbers">
    <li>{{ idx }}: {{ item }}</li>
  </ul>
</template>

<style></style>
