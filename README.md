# Modul-9-Software-Architectures

<details>
<summary>Subscriber</summary>

AMQP (Advanced Message Queuing Protocol) adalah protokol standar yang digunakan untuk mengirim pesan antar aplikasi dengan jaminan keamanan dan keandalan. Protokol ini memungkinkan aplikasi untuk berkomunikasi secara asinkron melalui message broker.

Dalam connection string `amqp://guest:guest@localhost:5672`, "guest" yang pertama adalah username untuk autentikasi ke message broker, "guest" yang kedua adalah password untuk login, sedangkan `localhost:5672` merupakan alamat host dan port di mana message broker (biasanya RabbitMQ) berjalan, dengan 5672 sebagai port default AMQP.

</details>

<details>
<summary>Publisher</summary>

Program `publisher` di folder `publisher` memanggil `publish_event` sebanyak lima kali, jadi dalam satu run ia mengirim tepat lima pesan `UserCreatedEventMessage`. Setiap pesan berisi `user_id` dan `user_name`; ukuran serialisasi tiap pesan bergantung pada panjang string (umumnya beberapa puluh sampai beberapa ratus byte), sehingga total data kira-kira 5 × ukuran_per_pesan.

Ya, publisher dan subscriber menggunakan URL yang sama, artinya keduanya terhubung ke message broker yang sama (`localhost` pada port `5672`) dengan kredensial yang sama (`guest` sebagai username dan `guest` sebagai password), sehingga pesan yang dipublikasikan oleh publisher tersedia bagi subscriber yang juga terhubung ke broker itu.

</details>

## RabbitMQ Running
![RabbitMQ Running](assets/images/rabbitmq-running.png)

## Sending and Processing Event
![Subscriber](assets/images/send-process-subscriber.png)
![Publisher](assets/images/send-process-publisher.png)
Publisher mengirimkan pesan event ke RabbitMQ dengan merangkaian data pengguna menggunakan Borsh dan mempublikasikannya ke queue user_created. Sementara itu, Subscriber mendengarkan dan mengonsumsi pesan dari queue yang sama, kemudian mendeserialisasi data untuk memproses setiap event pengguna yang diterima.

## Monitoring Chart based on Publisher
![Monitoring](assets/images/monitoring-publisher.png)
Spike yang terlihat pada grafik monitoring RabbitMQ menunjukkan lonjakan jumlah pesan saat publisher menjalankan `publish_event` lima kali berturut-turut, menyebabkan lima pesan masuk ke queue `user_created` dalam waktu singkat.
