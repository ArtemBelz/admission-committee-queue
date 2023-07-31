use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use teloxide::prelude::*;

use crate::dialogue::states::ReceiveIntervalState;
use crate::dialogue::Dialogue;
use crate::queue::Queue;

#[derive(Clone, Serialize, Deserialize)]
pub struct ReceiveDayState;

#[teloxide(subtransition)]
async fn receive_day(
    state: ReceiveDayState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match NaiveDate::parse_from_str(&format!("{}.2023", ans), "%d.%m.%Y") {
        Ok(date) => match Queue::global().get_intervals_keyboard(date).await {
            Ok(keyboard) => {
                cx.answer("Оберіть проміжок часу")
                    .reply_markup(keyboard)
                    .await?;
                next(Dialogue::ReceiveInterval(ReceiveIntervalState { date }))
            }
            Err(error) => {
                cx.answer(error.to_string()).await?;
                next(Dialogue::ReceiveDay(state))
            }
        },
        Err(_) => {
            cx.answer("Введено невірний формат дня").await?;
            next(Dialogue::ReceiveDay(state))
        }
    }
}
