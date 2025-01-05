# TP2 - Grupo 10 - [KWIC - KickForward]
Trabalho do Grupo 10 de Técnicas de Programação 2, implementando o KWIC com o estilo de programação Kick Forward em Rust.

**Especificação da Primeira Avaliação**: Exercises in Programming Style
Esta avaliação em grupo tem como objetivo explorar estilos de programação descritos no livro Exercises in Programming Style, da Profa. Crista Lopes. Após a entrega deste trabalho, os alunos devem se sentir mais familiarizados com a relevância dos diferentes estilos de programação, e ter domínio sobre um estilo de programação específico (escolhido pelo o grupo). 

**A atividade consiste em:**
1. Implementar o keyword in context (KWIC) utilizando o estilo e a linguagem de programação escolhidos. A especificação do KWIC está disponível no final deste post.
2. Implementar testes unitários e de integração para os principais componentes do KWIC. Testes automatizados consiste em um critério importante desta avaliação (e da disciplina como um todo).
3. Elaborar uma vídeo aula sobre o estilo de programação escolhido, reforçando como tal estilo influenciou a implementação do KWIC. A vídeo aula deve seguir a estrutura do capítulo do livro que descreve o estilo de programação escolhido, incluindo constraints, exemplo de como o KWIC foi implementado usando o estilo de programação escolhido, detalhes mais relevantes do estilo que impactou o desenho do KWIC no estilo escolhido,  como o estilo de programação é usado no desenho de sistemas e notas históricas.

**Entrega:**
Enviar arquivo texto ou markdown contendo:
- nome e matrícula dos membros do grupo,
- estilo e linguagem de programação escolhidos, 
- link para o repositório no GitHub com o código do projeto
- link para a vídeo alula
- informações sobre como fazer o build e executar os testes do projeto

**Especificação do Keyword in Context**
 
**Objetivo:**
- O objetivo deste projeto é implementar o algoritmo Keyword in Context (KWIC). Este algoritmo recebe uma lista de títulos ou frases e gera uma lista alfabetizada de todas as palavras-chave nesses títulos, juntamente com seu contexto circundante. Este é um exemplo clássico de manipulação e ordenação de strings, útil para construir índices e concordâncias.
 
**Entrada:**
- Um arquivo de texto contendo uma lista de títulos ou frases, um por linha.

**Saída:**
- Uma lista alfabetizada de palavras-chave, onde cada palavra-chave é apresentada com seu contexto circundante dentro de sua frase original.
  
Etapas do Algoritmo:

1. Ler Entrada: Leia o arquivo de texto de entrada e armazene os títulos em uma lista.
2. Gerar Lista de Palavras-chave:
  - Para cada título:
    - Divida o título em palavras individuais (palavras-chave).
    - Ignore palavras comuns ("stop words") como "a," "o," "as," "os," "um," "uma," "é," "de," etc. (Forneça uma lista predefinida de stop words).
    - Armazene cada palavra-chave junto com seu título original.
3. Deslocamento Circular: Para cada palavra-chave, crie uma versão "circularmente deslocada" de seu título onde a palavra-chave aparece no início.
  - Exemplo: Para o título "The quick brown fox" e a palavra-chave "brown", a versão circularmente deslocada é "brown fox The quick".
4. Ordenar: Ordene a lista de palavras-chave em ordem alfabética, com base nos títulos circularmente deslocados.
5. Saída:  Imprima a lista ordenada de palavras-chave e seus contextos (títulos circularmente deslocados) no console ou em um novo arquivo de texto.


**Exemplo:**
 
**Entrada:**
```
The quick brown fox
A brown cat sat
The cat is brown
```

**Saída:**
```
brown cat sat A          (from "A brown cat sat")
brown fox The quick      (from "The quick brown fox")
brown is The cat         (from "The cat is brown")
cat is brown The         (from "The cat is brown")
cat sat A brown          (from "A brown cat sat")
fox The quick brown      (from "The quick brown fox")
quick brown fox The      (from "The quick brown fox")
```
 
**Extensões opcionais:**
Permitir que os usuários especifiquem sua própria lista de stop words.
Implementar diferentes opções de ordenação (por exemplo, sensível a maiúsculas e minúsculas vs. insensível a maiúsculas e minúsculas).
Permitir que os usuários especifiquem o tamanho da janela de contexto ao redor da palavra-chave.
Criar uma interface gráfica do usuário (GUI) para o programa.
 
 
 
