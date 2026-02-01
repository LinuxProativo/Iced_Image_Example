# Iced Image Example

Exemplo mínimo de como carregar e exibir imagens corretamente no **iced**, utilizando `iced::widget::image::Handle` de acordo com o fluxo `update → state → view`.

## Objetivo

Demonstrar o padrão recomendado para uso de imagens no iced:

- carregar a imagem fora da função `view`
- armazenar o `image::Handle` no estado da aplicação
- reutilizar o `Handle` nas renderizações seguintes

## Estrutura

- a imagem é carregada uma única vez durante a atualização do estado
- o estado mantém o `image::Handle`
- a função `view` apenas consome o estado para descrever a interface

## Código principal

O fluxo essencial é:

1. o estado começa sem imagem
2. no `update`, o arquivo é lido e o `Handle` é criado
3. o `Handle` é salvo no estado
4. na `view`, o `Handle` é clonado e passado ao widget `Image`

Esse padrão evita operações de sistema durante a renderização e mantém a `view` como uma função puramente declarativa.

## Executando

Coloque uma imagem em:

```text
assets/image.png
```

E execute o projeto normalmente:

```bash
cargo run
```

## Observações

- `Handle::clone()` é barato e não duplica a imagem
- a `view` pode ser chamada muitas vezes; por isso ela não deve carregar arquivos
- o iced reconstrói a árvore de widgets, mas atualiza apenas o que mudou

## Licença

Este exemplo é livre para uso e adaptação.

