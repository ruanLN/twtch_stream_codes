#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

// uma lista eh:
// - Vazio, ou
// - Um elemento e mais uma lista

// List = Empty | Elem(x, List)

// (cons 1 (cons 2 empty))

typedef struct lista {
    int elemento;
    struct lista *proximo;
} Lista;

typedef struct embrulho_da_lista
{
    Lista *lista_interna;
} EmbrulhoDaLista;

typedef struct resultado {
    int resultado;
    int sucesso;
} Resultado;


Lista* criar() {
    return NULL;
}

Lista* adicionar_inicio(Lista* lista, int elemento) {
    if (lista == NULL) {
        lista = calloc(1, sizeof(Lista));
        lista->proximo = NULL;
        lista->elemento = elemento;
        return lista;
    } else {    // lista tem elementos
        Lista *nova_lista = calloc(1, sizeof(Lista));
        nova_lista->elemento = elemento;
        nova_lista->proximo = lista;
        return nova_lista;
    }
}

// remove a primeira ocorrencia do elemento na lista
Lista* remover(Lista* lista, int elemento) {
     if (lista == NULL) {
         return lista;
     } else {
         if (lista->elemento == elemento) {
             Lista *proxima = lista->proximo;
             free(lista);
             return lista->proximo;
         } else {   
             lista->proximo = remover(lista->proximo, elemento);
             return lista;
         }
     }
}

Lista* remover_indice(Lista* lista, int indice);

Resultado obter_indice(Lista* lista, int indice) {
    Resultado resultado;
    resultado.resultado = 0;
    resultado.sucesso = 0;

    for (int i = 0; i < indice && lista != NULL; i++) {
        lista = lista->proximo;
    }
    if (lista != NULL) {
        resultado.resultado = lista->elemento;
        resultado.sucesso = 1;
    }
    return resultado;
}

void imprimir(Lista* lista) {
    while (lista != NULL){
        printf("%d -> ", lista->elemento);
        lista = lista->proximo;
    }
    printf("NULL\n");
}

int main(int argc, char const *argv[])
{
    Lista* lista = criar();
    imprimir(lista);
    assert(obter_indice(lista, 5).sucesso == 0);
    assert(obter_indice(lista, 0).sucesso == 0);

    lista = adicionar_inicio(lista, 7);
    imprimir(lista);

    assert(obter_indice(lista, 0).sucesso == 1);
    assert(obter_indice(lista, 0).resultado == 7);

    assert(obter_indice(lista, 3).sucesso == 0);

    lista = adicionar_inicio(lista, 23);
    imprimir(lista);

    assert(obter_indice(lista, 0).sucesso == 1);
    assert(obter_indice(lista, 0).resultado == 23);
    assert(obter_indice(lista, 1).sucesso == 1);
    assert(obter_indice(lista, 1).resultado == 7);
    
    assert(obter_indice(lista, 3).sucesso == 0);


    lista = remover(lista, 11);
    imprimir(lista);

    lista = remover(lista, 23);
    imprimir(lista);

    lista = adicionar_inicio(lista, 23);
    imprimir(lista);
    lista = remover(lista, 7);
    imprimir(lista);


    printf("All tests passed\n");
    return 0;
}
