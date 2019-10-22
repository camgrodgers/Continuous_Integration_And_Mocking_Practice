pipeline {
	agent {
        docker { image 'rust' }
    }

    stages {
        stage('Build') {
            steps {
                sh 'rustup default stable'
                sh 'cargo build --release'
            }
        }
        stage('UnitTest') {
            steps {
                sh 'cargo test unit'
            }
        }
        stage('DummyTest') {
            steps {
                sh 'cargo test mock'
            }
        }
        stage('HttpTest') {
            steps {
                sh 'cargo test http'
            }
        }
	}
}

