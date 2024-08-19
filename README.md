# Segurança Computacional 2024.1 - Trabalho de Implementação
Henrique Rodrigues Rocha - 211036061

Este trabalho foi desenvolvido na linguagem de programação [Rust](https://www.rust-lang.org/pt-BR) como tarefa da
disciplina de Segurança Computacional (CIC0201) na Universidade de Brasília (UnB).

## Parte 1 - Cifra de bloco e modo de operação CTR

Implementação do algoritmo de criptografia simétrica
[AES-128](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) com o modo de operação
[CTR](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Counter_(CTR)).

### Uso

Devem ser fornecidos os seguintes argumentos:

* \<input\> - o caminho do arquivo de entrada
* \<output\> - o caminho do arquivo de saída
* \<key\> - a chave de 128 bits codificada como um inteiro sem sinal
* \<iv\> - o vetor inicial (offset) codificado como um inteiro sem sinal
* \<rounds\> - a quantidade de rounds, opcional, 10 por padrão

Por exemplo:

```shell
aes plaintext.txt ciphertext.txt 20011376718272490338853433276725592320 0 10
```

## Parte 2 - Gerador/Verificador de Assinaturas
