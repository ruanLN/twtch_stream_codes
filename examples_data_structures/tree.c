struct arvgen
{
    char info;
    struct arvgen *primeiro_filho;
    struct arvgen *proximo_irmao;
};
typedef struct arvgen ArvGen;
ArvGen *cria(char c)
{
    ArvGen *a = (ArvGen *)malloc(sizeof(ArvGen));
    a->info = c;
    a->primeiro_filho = NULL;
    a->proximo_irmao = NULL;
    return a;
}

void insere(ArvGen *a, ArvGen *sa)
{
    sa->proximo_irmao = a->primeiro_filho;
    a->primeiro_filho = sa;
}

/*
ArvGen *a = cria('a');
ArvGen *b = cria('b');
ArvGen *c = cria('c');
insere(a, b);
insere(a ,c);

a->primeiro_filho = c
c -> proximo_irmao = b
*/

void imprime(ArvGen *raiz)
{
    ArvGen *p;
    printf("%c\n", raiz->info);
    for (p = raiz->primeiro_filho; p != NULL; p = p->proximo_irmao)
        imprimeiro_file(p);
}

ArvGen *adicione_no_final(ArvGen *a, ArvGen *b) 
{
    ArvGen *p;
    for (p = a; p->proximo_irmao != NULL; p = p->proximo_irmao) {}
    p->proximo_irmao = b;
        

    return a;
}

ArvGen *retira(ArvGen *a, char v)
{
    if (a == NULL)
    {
        return NULL;
    }
    else if (a->info != v)
    {
        a->primeiro_filho = retira(a->primeiro_filho, v);
        a->proximo_irmao = retira(a->proximo_irmao, v);
        return a;
    }
    else
    {
        if (a->primeiro_filho == NULL && a->proximo_irmao == NULL)
        {
            free(a);
            return NULL;
        }
        else if (a->primeiro_filho == NULL)
        {
            ArvGen *t = a;
            a = a->proximo_irmao;
            free(t);
            return a;
        }
        else if (a->proximo_irmao == NULL)
        {
            ArvGen *t = a;
            a = a->primeiro_filho;
            free(t);
            return a;
        }
        else
        {
            ArvGen *t = a;
            a = a->primeiro_filho;
            a = adicione_no_final(a, t->proximo_irmao);
            free(t);
            return a;
        }
        
    }
}