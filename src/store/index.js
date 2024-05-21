import { createStore } from 'vuex'
const store =createStore({
    state(){
        return{
         path:'E:/Office'
        }
    },
    mutations:{
        changepath(state, path){
            state.path = path;
        }
    },
})

export default store;