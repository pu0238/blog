<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let blogs = ref([]);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = target.querySelector('#title').value;
  const content = target.querySelector('#content').value;
  const tags = target.querySelector('#tags').value;
  const splitedTags = tags.split(",")

  await my_project_backend.add_blog(title, content, splitedTags)
  await getBlogs()
}

async function getBlogs() {
  const tempBlogs = await my_project_backend.get_blogs()
  blogs.value = tempBlogs.map((blog) => {
    return {
      ...blog,
      date: blog.date.toString()
    }
  })
}
getBlogs()
</script>

<template>
  <main class="container mx-auto">
    <img src="/logo2.svg" alt="DFINITY logo" class="mx-auto mt-4"/>
    <br />
    <br />
    <form class="grid gap-4 pb-4 mb-4 border-solid border-b-2 border-sky-500" action="#" @submit="handleSubmit">
      <div>
        <p class="text-black font-bold">Title: </p>
        <input 
          id="title" 
          alt="title" 
          type="text" 
          class="
            w-full 
            rounded-3xl 
            py-1 
            px-4 
            outline-none
            border-solid 
            border-2
            border-black
            hover:border-indigo-500
            "/>
      </div>
      <div>
        <p class="text-black font-bold ">Content: </p>
        <textarea
          id="content" 
          alt="content" 
          type="text" 
          class="
            w-full 
            rounded-3xl 
            py-1 
            px-4 
            outline-none 
            min-h-[100px] 
            border-solid 
            border-2
            border-black
            hover:border-indigo-500
          "></textarea>
      </div>
      <div>
        <p class="text-black font-bold">Tags: </p>
        <input 
          id="tags" 
          alt="tags" 
          type="text" 
          class="
            w-full 
            rounded-3xl 
            py-1 
            px-4 
            outline-none
            border-solid 
            border-2
            border-black
            hover:border-indigo-500"
          />
      </div>
      <div class="flex justify-end">
        <button 
          class="
          text-white 
          bg-indigo-400 
          rounded-3xl 
          py-1 
          px-4" type="submit">
            Click to add!
        </button>
      </div>
    </form>
    <div>
      <div v-for="blog in blogs"> 
        <h3>{{ blog.title }} </h3>
        <p>{{ blog.content }}</p>
        <div>
          {{ blog.date }}
          {{ blog.tags }}
        </div>
      </div>
    </div>
  </main>
</template>
