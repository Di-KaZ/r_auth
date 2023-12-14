import amaplib from 'amqplib';
import nodemailer from 'nodemailer';

interface NewUser {
  id: number;
  username: string;
  email: string;
  verified: boolean;
}

const queue = 'register';

const conn = await amaplib.connect(Bun.env.AMQP_URL!);

const channel = await conn.createChannel();

await channel.assertQueue(queue, { durable: false });

const transporter = nodemailer.createTransport({
  host: Bun.env.SMTP_HOST!,
  port: parseInt(Bun.env.SMTP_PORT!),
  secure: false,
  auth: {
    user: 'guest',
    pass: 'guest',
  }
});

channel.consume(queue, async (msg) => {
  if (msg != null) {
    const user: NewUser = JSON.parse(msg.content.toString());
    const res = await transporter.sendMail({
      from: "moussa.fofana@mailhog.local",
      to: user.email,
      subject: 'Email verification !',
      html: `
        <h1>Hello ${user.username}</h1>
        <p>We need to confirm your email, if this is requested please click on <a href="#">this link</a></p>
        -> The very bad rust dev thanks you for your support.
      `
    });
    console.log(res.messageId);
  } else {
    console.log('cancelled')
  }
});
