Questions
=============================================

## 1) How do I recognize that data is allocated at the heap rather than at the stack? When data should be allocated at the heap?

Дані що лежать на стеку зберігаються, якщо їхній розмір відомий на етапі компіляції (наприклад, числа чи масиви фіксованої довжини). А дані на купі зберігаються, якщо їх розмір
не фіксований або вони повинні жити довше за поточну функцію. Наприклад, Box, Vec, String виділяють пам'ять у купі. На купу варто переходити, якщо потрібно зберігати великі дані або їх потрібно передавати між функціями і потоками.

## 2) What is copying and cloning data in Rust? What's the difference? When and why should I use them?

• Копіювання — це коли прості дані, такі як числа або значення булеві, копіюються побітно. Це дуже швидко і не потребує додаткових витрат.

• Клонування — це складніша операція, яка копіює дані на купі. Наприклад, якщо у нас String, то клонування створить новий об'єкт із копією рядка.
Використовувати Copy – коли дані прості, а Clone – якщо потрібно створити повну копію складного об'єкта.

## 3) How can a single piece of data be owned by multiple parts of program? When and why is this commonly required?

Для цього використовується Rc (в однопотоковому коді) або Arc (у багатопотоковому). Вони дозволяють кільком частинам програми розділяти володіння даними. Це потрібно,
наприклад, якщо є загальний ресурс, який не можна переміщати, але його треба використовувати із різних місць. Наприклад, для побудови графів чи поділу завдань між потоками.

## 4) How borrowing rules may be violated? In what price? When and why is this commonly required?

Правила запозичення порушуються, якщо одночасно є змінюване або незмінне посилання або два змінні посилання. Іноді потрібно порушувати правила, щоб уникнути обмеження,
наприклад, використовуючи RefCell або unsafe, але це треба робити обережно, щоб не зламати безпеку програми.

## 5) How to deal with owned and borrowed data simultaneously? When and why is this commonly required?

Це складно, але можливо. Наприклад, можна використовувати посилання (&) або запозичувати через RefCell. Mutex<T> у багатопотоковому. Це буває потрібно, якщо дані повинні
змінюватись у кількох місцях.

## 6) How to share values between threads? What is Send and Sync markers? Why are they required, when should be used?

Щоб ділити дані між потоками, потрібно використовувати Arc та синхронізацію (наприклад, Mutex).

Send – це ознака того, що дані можна передавати в інший потік, а Sync – що безпечно використовувати дані з декількох потоків одночасно. Вони вбудовані в компілятор, щоб
уникнути помилок під час роботи з багатопоточністю.

## 7) How do static and dynamic dispatches differ? Why do they exist? When and why should I choose between them?

• Статична диспетчеризація відбувається на етапі компіляції, коли відомі типи. Це швидше, тому що виклик функції інлайнується.

• Динамічна диспетчеризація використовується, якщо тип функції невідомий заздалегідь. Це гнучко, але повільніше.

Вибір залежить від завдання: якщо продуктивність критична - статична. Якщо потрібна гнучкість - динамічна.

## 8) Why ?Sized types exist? How are they used? Why should I care about them?

?Sized - це типи, розмір яких невідомий на етапі компіляції. Наприклад, str або масиви [T] без довжини. Вони часто використовуються через посилання, наприклад &str або Box<dyn
Trait>. 

Це важливо, якщо потрібно працювати з даними довільного розміру, але зберігати безпеку і оптимізацію.

## 9) Why phantom types exist? What problems do they solve?

Фантомні типи дозволяють додавати "віртуальні" типи, щоб компілятор міг перевіряти обмеження чи типобезопасность. Наприклад, якщо потрібно вказати, що структура працює з певним
типом, але сама її не зберігає. Це допомагає уникнути помилок і робить API зрозумілішим.