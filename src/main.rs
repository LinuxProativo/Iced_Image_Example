use iced::widget::image::Handle;
use iced::widget::{Column, Image, column};
use iced::{ContentFit, Element, Event, Subscription, Task, event, window, Size};

#[derive(Debug, Clone)]
enum Message {
    // Mensagem para o Update
    EventOccurred(Event),
}

#[derive(Debug, Default)]
struct State {
    // Agora Temos um Estado
    // O derive Default, garante que o estado vai começar com image: None.
    image: Option<Handle>,
}

pub fn main() -> iced::Result {
    // Aqui utilizamos uma struct de estado própria para armazenar a imagem.
    // Essa struct é capaz de manter dados entre ciclos da aplicação.
    iced::application(State::default, update, view)
        .window(window::Settings {
            size: Size::new(400.0, 400.0),
            ..Default::default()
        })
        .subscription(subscription)
        .run()
}

// Os eventos são capturados pela Subscription
fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::batch(vec![
        // Os evento são enviados para o Update.
        event::listen().map(Message::EventOccurred)
    ])
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::EventOccurred(_event) => {
            // IMPORTANTE: Verifica se o Image ainda está nulo, o que significa
            // que a imagem ainda não foi criada.
            if state.image.is_none() {
                let path = std::path::PathBuf::from("assets/image.png");

                if let Ok(bytes) = std::fs::read(path) {
                    // A leitura do arquivo só ocorre durante a atualização do estado.
                    // O Handle é criado apenas uma vez e salvo no estado.
                    state.image = Some(Handle::from_bytes(bytes));
                }
            }

            Task::none()
        }
    }
}

// Após o update, a função view é chamada.
fn view(state: &State) -> Column<Message> {
    let icon_widget: Element<Message> = if let Some(handle) = &state.image {
        // O widget Image é criado a partir de um clone do Handle armazenado.
        // A Column é construída usando apenas os dados presentes no estado.
        // Em execuções futuras da view, o mesmo Handle armazenado é reutilizado.
        // Quando o View reconstruir a interface e o iced não vai redesenhar a imagem.
        Image::new(handle.clone())
            .content_fit(ContentFit::ScaleDown)
            .width(512)
            .height(512)
            .into()
    } else {
        iced::widget::text("").into()
    };

    column![
        icon_widget,
    ]
        .into()
}
