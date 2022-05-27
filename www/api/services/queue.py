import pika
from api.config import MSG_QUEUE_HOST


class QueueProducer(object):

    instance = None

    def __init__(self, connection_string: str):
        pass
        # self.connection = pika.BlockingConnection(pika.ConnectionParameters(connection_string))
        # self.channel = self.connection.channel()
        # self.channel.exchange_declare(exchange='drepr', exchange_type='fanout')

    def __del__(self):
        pass
        # if 'connection' in self.__dict__:
        #     self.connection.close()

    @staticmethod
    def get_instance():
        if QueueProducer.instance is None:
            QueueProducer.instance = QueueProducer(MSG_QUEUE_HOST)
        return QueueProducer.instance

    def publish(self, msg: str):
        pass
        # self.channel.basic_publish(exchange='drepr', routing_key='', body=msg)


if __name__ == '__main__':
    producer = QueueProducer.get_instance()
    for i in range(10):
        print(producer.publish(f"hello {i}"))
